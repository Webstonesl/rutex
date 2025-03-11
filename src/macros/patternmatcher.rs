use std::hash::Hash;
use std::{collections::HashMap, fmt::Debug, ops::Range};

#[derive(Debug)]
pub enum PatternMatchResult<T>
where
    T: Eq + Debug + Hash,
{
    Found(HashMap<T, Range<usize>>),
    Never,
    Possible,
}

#[derive(Debug)]
pub enum MatcherElement<'a, T, El>
where
    T: Eq + Clone + Debug + Hash,
    El: Eq + Debug,
{
    Literal(&'a El),
    Capture(T, usize),
}
impl<'a, T, El> PartialEq for MatcherElement<'a, T, El>
where
    T: Eq + Clone + Debug + Hash,
    El: Eq + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            _ => false,
        }
    }
}
#[derive(Debug)]
enum PatternItem<'a, T>
where
    T: Eq + Clone + Debug + Hash,
{
    Literal {
        range: Range<usize>,
        locations: Vec<usize>,
    },
    Parameter {
        key: &'a T,
        min_length: usize,
        min_location: usize,
        max_location: usize,
        preceded_by_parameter: bool,
        followed_by_parameter: bool,
    },
}

pub struct PatternMatcher<'a, T, El>(&'a [MatcherElement<'a, T, El>], usize)
where
    T: Eq + Clone + Debug + Hash,
    El: Eq + Debug;

impl<'a, T, El> PatternMatcher<'a, T, El>
where
    T: Eq + Debug + Clone + Hash,
    El: Eq + PartialEq<El> + Debug,
{
    pub fn new(pattern: &'a [MatcherElement<'a, T, El>]) -> Self {
        Self(
            pattern,
            pattern
                .iter()
                .map(|p| match p {
                    MatcherElement::Literal(_) => 1,
                    MatcherElement::Capture(_, n) => *n,
                })
                .sum(),
        )
    }

    pub fn find_match(&self, target: &[El]) -> PatternMatchResult<T> {
        if target.len() < self.1 {
            return PatternMatchResult::Never;
        }
        let mut literal_ranges: HashMap<Range<usize>, Vec<usize>> = HashMap::new();
        let mut literal_range: Option<Range<usize>> = None;
        for (i, e) in self.0.iter().enumerate() {
            match (&literal_range, e) {
                (None, MatcherElement::Literal(_)) => literal_range = Some(i..(i + 1)),
                (None, MatcherElement::Capture(_, _)) => {}
                (Some(Range { start, .. }), MatcherElement::Literal(_)) => {
                    literal_range = Some(*start..(i + 1));
                }
                (Some(r), MatcherElement::Capture(_, _)) => {
                    literal_ranges.insert(r.clone(), Vec::new());
                    literal_range = None;
                }
            }
        }
        if let Some(Range { start, end }) = literal_range {
            literal_ranges.insert(Range { start, end }, Vec::new());
        }
        for (k, vec) in literal_ranges.iter_mut() {
            let a = &self.0[k.clone()];
            'current_position: for initial_index in 0..=(target.len() - a.len()) {
                for i in 0..a.len() {
                    let r = if let MatcherElement::Literal(el) = a[i] {
                        el == &target[initial_index + i]
                    } else {
                        false
                    };
                    if !r {
                        continue 'current_position;
                    }
                }
                vec.push(initial_index);
            }
        }
        let mut items = Vec::new();
        let mut i = 0;
        while i < self.0.len() {
            for (k, v) in literal_ranges.iter() {
                if k.contains(&i) {
                    items.push(PatternItem::Literal {
                        range: k.clone(),
                        locations: v.clone(),
                    });
                    i = k.end;
                }
            }
            if let Some(MatcherElement::Capture(t, l)) = self.0.get(i) {
                items.push(PatternItem::Parameter {
                    key: t,
                    min_length: *l,
                    min_location: 0,
                    max_location: target.len(),
                    preceded_by_parameter: false,
                    followed_by_parameter: false,
                });
                i += 1;
                continue;
            }
            unreachable!("Should not be reachable");
        }
        {
            let mut last_end = 0;
            let mut prev_is_parameter = false;
            for item in items.iter_mut() {
                match item {
                    PatternItem::Literal { range, locations } => {
                        prev_is_parameter = false;
                        if range.start == 0 {
                            locations.retain(|&f| f == 0);
                        } else {
                            locations.retain(|&location| location >= last_end);
                        }
                        if let Some(&start) = locations.iter().min() {
                            last_end = start + range.len();
                        } else {
                            return if range.start == 0 {
                                PatternMatchResult::Never
                            } else {
                                PatternMatchResult::Possible
                            };
                        }
                    }
                    PatternItem::Parameter {
                        min_length,
                        min_location,
                        preceded_by_parameter,
                        ..
                    } => {
                        *min_location = last_end;
                        last_end += *min_length;
                        *preceded_by_parameter = prev_is_parameter;
                        prev_is_parameter = true;
                    }
                }
            }
            let mut last_start = target.len();
            let mut next_is_parameter = false;
            for item in items.iter_mut().rev() {
                match item {
                    PatternItem::Literal { range, locations } => {
                        next_is_parameter = false;
                        locations.retain(|&location| location + range.len() <= last_start);
                        if let Some(&start) = locations.iter().max() {
                            last_start = start;
                        } else {
                            return if range.start == 0 {
                                PatternMatchResult::Never
                            } else {
                                PatternMatchResult::Possible
                            };
                        }
                    }
                    PatternItem::Parameter {
                        min_length,
                        max_location,
                        followed_by_parameter,
                        ..
                    } => {
                        *max_location = last_start;
                        last_start -= *min_length;
                        *followed_by_parameter = next_is_parameter;
                        next_is_parameter = true;
                    }
                }
            }
            for item in items.iter_mut() {
                if let PatternItem::Literal { range, locations } = item {
                    let &l = locations.iter().min().unwrap();
                    *range = l..(range.len() + l);
                }
            }
            let mut last = 0;
            for item in items.iter_mut() {
                match item {
                    PatternItem::Literal { range, .. } => {
                        last = range.end;
                    }
                    PatternItem::Parameter {
                        min_length,
                        min_location,
                        ..
                    } => {
                        *min_location = last;
                        last = *min_location + *min_length;
                    }
                };
            }

            for item in items.iter_mut().rev() {
                match item {
                    PatternItem::Literal { range, .. } => {
                        last = range.start;
                    }
                    PatternItem::Parameter {
                        max_location,
                        min_location,
                        ..
                    } => {
                        *max_location = last;
                        last = *min_location;
                    }
                };
            }
        }
        let mut result = HashMap::new();
        for item in items {
            if let PatternItem::Parameter {
                key,
                min_location,
                max_location,
                ..
            } = item
            {
                result.insert(key.clone(), min_location..max_location);
            }
        }
        return PatternMatchResult::Found(result);
    }
}
#[cfg(test)]
#[derive(PartialEq, Eq, Debug)]
enum TestElement {
    Parameter(usize),
    LiteralChar(char),
}
#[cfg(test)]
impl TestElement {
    fn from_string(s: &str) -> Vec<TestElement> {
        let mut v = vec![];
        let mut i = 0;
        let chars: Vec<char> = s.chars().collect();

        while i < s.len() {
            match chars[i] {
                '#' => {
                    let mut j = i + 1;
                    while let Some('0'..'9') = chars.get(j) {
                        j += 1;
                    }

                    i = i + 1;
                    if i == j {
                        panic!("{} = {} {:?}", i, j, s);
                    }
                    v.push(TestElement::Parameter(s[i..j].parse().unwrap()));
                    i = j;
                }
                a => {
                    v.push(TestElement::LiteralChar(a));
                    i += 1;
                }
            }
        }

        v
    }
}

#[test]
fn test() {
    let a = TestElement::from_string("(#1#2#3)(#4#5)(#6)#7");
    let b: Vec<char> = "(apples)(pears)(aso))".chars().collect();
    let pattern: Vec<MatcherElement<'_, &usize, char>> = {
        a.iter()
            .map(|a| match a {
                TestElement::LiteralChar(a) => MatcherElement::Literal(a),
                TestElement::Parameter(n) => MatcherElement::Capture(n, 1),
            })
            .collect()
    };
    dbg!(&pattern);
    let matcher = PatternMatcher::new(&pattern);
    dbg!(matcher.find_match(&b));
}
