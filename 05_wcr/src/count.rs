use std::iter::Sum;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

impl Add for Count {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Count {
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            chars: self.chars + rhs.chars,
            bytes: self.bytes + rhs.bytes,
        }
    }
}
impl Add<&Self> for Count {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Count {
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            chars: self.chars + rhs.chars,
            bytes: self.bytes + rhs.bytes,
        }
    }
}

impl<'a> Sum<&'a Self> for Count {
    fn sum<I>(iter: I) -> Count
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Count::default(), Count::add)
    }
}

#[cfg(test)]
mod test {
    use super::Count;

    #[test]
    fn add() {
        let count = Count {
            lines: 1,
            words: 2,
            chars: 3,
            bytes: 4,
        };

        assert_eq!(count, Count::default() + count);
    }

    #[test]
    fn add_ref() {
        let count = Count {
            lines: 1,
            words: 2,
            chars: 3,
            bytes: 4,
        };

        assert_eq!(count, Count::default() + &count);
    }

    #[test]
    fn sum() {
        let counts = vec![
            Count {
                lines: 1,
                ..Count::default()
            },
            Count {
                words: 1,
                ..Count::default()
            },
            Count {
                chars: 1,
                ..Count::default()
            },
            Count {
                bytes: 1,
                ..Count::default()
            },
        ];

        let expected = Count {
            lines: 1,
            words: 1,
            chars: 1,
            bytes: 1,
        };

        let actual: Count = counts.iter().sum();

        assert_eq!(expected, actual);
    }
}
