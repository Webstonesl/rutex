use std::{boxed::Box, cmp::Ordering, fmt::Debug};

struct BinaryHeapInner<T> {
    value: T,
    left: Option<Box<BinaryHeapInner<T>>>,
    right: Option<Box<BinaryHeapInner<T>>>,
    count: usize,
}
impl<T> BinaryHeapInner<T> {
    fn insert_into<F: FnMut(&T, &T) -> Ordering>(&mut self, value: T, cmp: &mut F) -> bool {
        let ord = cmp(&self.value, &value);
        let v = match ord {
            Ordering::Less => &mut self.right,
            Ordering::Equal => return false,
            Ordering::Greater => &mut self.left,
        };
        match v {
            Some(a) => {
                if a.insert_into(value, cmp) {
                    self.count += 1;
                }
            }
            a @ None => {
                *a = Some(Box::new(BinaryHeapInner {
                    value,
                    left: None,
                    right: None,
                    count: 1,
                }))
            }
        };
        true
    }
}
impl<T: Debug> BinaryHeapInner<T> {
    fn _test(from: &Option<Box<Self>>) -> String {
        if let Some(a) = from {
            format!("{:#?}", a)
        } else {
            "None".to_string()
        }
    }
}

impl<T: Debug> Debug for BinaryHeapInner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "{:?}", self.value)?;
            if let Some(ref left) = self.left {
                writeln!(
                    f,
                    "\tLeft:  {:}",
                    format!("{:#?}", left).replace('\n', "\n\t")
                )?
            };
            if let Some(ref right) = self.right {
                writeln!(
                    f,
                    "\tRight: {:}",
                    format!("{:#?}", right).replace('\n', "\n\t")
                )?
            };
        } else {
            if let Some(ref a) = self.left {
                a.fmt(f)?;
                write!(f, ", ")?;
            }
            write!(f, "{:?}", self.value)?;
            if let Some(ref a) = self.right {
                write!(f, ", ")?;
                a.fmt(f)?;
            }
        }
        Ok(())
    }
}

pub struct BinaryHeap<T, F: FnMut(&T, &T) -> Ordering> {
    value: Option<BinaryHeapInner<T>>,
    ord: F,
}

impl<T: Debug, F: FnMut(&T, &T) -> Ordering> Debug for BinaryHeap<T, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.value {
                Some(ref a) => {
                    write!(f, "Value {:#?}", a)
                }
                None => write!(f, "Empty Binary Heap"),
            }
        } else if let Some(ref value) = self.value {
            write!(f, "[{:?}]", value)
        } else {
            write!(f, "[]")
        }
    }
}

impl<T, F: FnMut(&T, &T) -> Ordering> BinaryHeap<T, F> {
    pub fn new(ord: F) -> Self {
        Self { value: None, ord }
    }
    pub fn len(&self) -> usize {
        if let Some(ref a) = self.value {
            a.count
        } else {
            0
        }
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_some()
    }

    pub fn push(&mut self, new_value: T) {
        let Self { value, ord } = self;
        match value {
            Some(a) => {
                a.insert_into(new_value, ord);
            }
            a @ None => {
                *a = Some(BinaryHeapInner {
                    value: new_value,
                    left: None,
                    right: None,
                    count: 1,
                })
            }
        }
    }
    pub fn from_iterator<I: IntoIterator<Item = T>>(iter: I, ord: F) -> Self {
        let mut v = Self::new(ord);
        for item in iter {
            v.push(item);
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sequencial() {
        let sample = [6, 8, 3, 2, 1, 9, 10, 5, 4, 7];
        let bheap = BinaryHeap::from_iterator(sample, Ord::cmp);
        eprintln!("{:#?}", bheap);
    }
}
