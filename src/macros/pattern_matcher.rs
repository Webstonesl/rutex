use std::{
    fmt::Debug,
    ops::{Add, Rem},
};

use crate::{errors::Error, parser::parser::Token, TexState};

struct OptionalZipper<A, B, C, D>
where
    A: Iterator<Item = C>,
    B: Iterator<Item = D>,
{
    a: A,
    b: B,
}
impl<A, B, C, D> OptionalZipper<A, B, C, D>
where
    A: Iterator<Item = C>,
    B: Iterator<Item = D>,
{
    fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}
impl<A, B, C, D> Iterator for OptionalZipper<A, B, C, D>
where
    A: Iterator<Item = C>,
    B: Iterator<Item = D>,
{
    type Item = (Option<C>, Option<D>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            a => Some(a),
        }
    }
}

struct PatternMatchResult<T> {
    map: Vec<Vec<T>>,
}
impl<T> PatternMatchResult<T> {
    fn new(n: u8) -> Self {
        Self {
            map: Vec::with_capacity(n.into()),
        }
    }
}

#[derive(Debug)]
enum Section<'a, T: PartialEq> {
    Constants(&'a [T]),
    Parameter(u8, bool),
}
impl<'a, T> Section<'a, T>
where
    T: PartialEq,
{
    fn matches(&self, tokens: &'a [T], min_length: usize) -> Result<(&'a [T], &'a [T]), ()> {
        match self {
            Self::Constants(a) => {
                for (target, actual) in OptionalZipper::new(a.iter(), tokens.iter()) {
                    if target != actual {
                        return Err(());
                    }
                }
                Ok((&tokens[..a.len()], &tokens[a.len()..]))
            }
            Self::Parameter(_, a) => {
                let min_length = if min_length == 0 && *a { 1 } else { min_length }.into();
                if tokens.len() < min_length {
                    Err(())
                } else {
                    Ok((&tokens[..min_length], &tokens[min_length..]))
                }
            }
        }
    }
}
fn try_with<'a, T: PartialEq + Debug>(
    target: &'a [Section<'a, T>],
    mut actual: &'a [T],
    lengths: &'a [usize],
) -> Result<PatternMatchResult<T>, ()> {
    let tagetlen: usize = target
        .iter()
        .map(|s| match s {
            Section::Constants(items) => items.len(),
            Section::Parameter(n, a) => lengths[*n as usize].max(if *a { 1 } else { 0 }),
        })
        .sum();
    if actual.len() < tagetlen {
        return Err(());
    }
    for t in target {
        let a: &'a [T];
        (a, actual) = match t {
            t @ Section::Constants(_) => t.matches(actual, 0)?,
            t @ Section::Parameter(parn, _) => t.matches(actual, lengths[*parn as usize])?,
        };
        dbg!(a);
    }

    todo!();
}
trait Number: Add<Output = Self> + PartialEq + PartialOrd + Copy + Clone + Rem<Output = Self> {}
impl Number for usize {}
struct Counter<T>
where
    T: Number,
{
    count: usize,
    current: Vec<T>,
    maximum: T,
    rollover: T,
    increase: T,
}
impl<T> Counter<T>
where
    T: Number,
{
    fn new(count: usize, maximum: T, initial: T, increase: T) -> Counter<T> {
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push(initial);
        }
        Counter {
            count,
            maximum,
            current: v,
            rollover: initial,
            increase,
        }
    }
}
impl<T> Iterator for Counter<T>
where
    T: Number,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        for m in (0..self.count).rev() {
            let mut nv = self.current[m] + self.increase;
            if nv >= self.maximum {
                if m == 0 {
                    return None;
                }
                nv = nv % self.maximum;
                self.current[m] = nv;
            } else {
                self.current[m] = nv;
                break;
            }
        }
        Some(self.current.clone())
    }
}

pub fn match_pattern<'a>(target: &'a Vec<Token>, state: &mut TexState) -> Result<(), Error> {
    todo!();
}
