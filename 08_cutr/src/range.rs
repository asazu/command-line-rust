use std::cmp::Ordering;
use std::num::NonZeroUsize;

pub mod parser;

mod iter;
pub use iter::RangeFilterEx;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range {
    Between(NonZeroUsize, NonZeroUsize),
    To(NonZeroUsize),
    From(NonZeroUsize),
}

impl Range {
    fn start(&self) -> usize {
        match self {
            Range::Between(start, _) => start.get(),
            Range::To(_) => 1,
            Range::From(start) => start.get(),
        }
    }

    fn end(&self) -> usize {
        match self {
            Range::Between(_, end) => end.get(),
            Range::To(end) => end.get(),
            Range::From(_) => usize::MAX,
        }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Range) -> Ordering {
        let left = (self.start(), self.end());
        let right = (other.start(), other.end());
        left.cmp(&right)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Range) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RangeList(Vec<Range>);

impl<T> From<T> for RangeList
where
    T: Into<Vec<Range>>,
{
    fn from(ranges: T) -> RangeList {
        let mut ranges = ranges.into();
        ranges.sort();
        RangeList(ranges)
    }
}

impl RangeList {
    fn iter(&self) -> std::slice::Iter<'_, Range> {
        self.0.iter()
    }
}

#[cfg(test)]
fn range_from(start: usize) -> Range {
    let start = NonZeroUsize::new(start).expect("start shoudl be non-zero");
    Range::From(start)
}

#[cfg(test)]
fn range_to(end: usize) -> Range {
    let end = NonZeroUsize::new(end).expect("end shoudl be non-zero");
    Range::To(end)
}

#[cfg(test)]
fn range_between(start: usize, end: usize) -> Range {
    let start = NonZeroUsize::new(start).expect("start shoudl be non-zero");
    let end = NonZeroUsize::new(end).expect("end shoudl be non-zero");
    assert!(start <= end);
    Range::Between(start, end)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_start() {
        assert_eq!(range_between(12, 34).start(), 12);
        assert_eq!(range_to(34).start(), 1);
        assert_eq!(range_from(12).start(), 12);
    }

    #[test]
    fn range_end() {
        assert_eq!(range_between(12, 34).end(), 34);
        assert_eq!(range_to(34).end(), 34);
        assert_eq!(range_from(12).end(), usize::MAX);
    }

    #[test]
    fn range_cmp() {
        let a = range_between(3, 5);
        let b = range_between(3, 4);
        let c = range_between(4, 5);
        let d = range_between(2, 3);
        let e = range_between(2, 4);
        let f = range_between(4, 6);
        let g = range_between(5, 6);
        let h = range_between(2, 6);
        let i = range_to(5);
        let j = range_from(3);

        assert_eq!(a.cmp(&a), Ordering::Equal, "a : a");
        assert_eq!(a.cmp(&b), Ordering::Greater, "a : b");
        assert_eq!(a.cmp(&c), Ordering::Less, "a : c");
        assert_eq!(a.cmp(&d), Ordering::Greater, "a : d");
        assert_eq!(a.cmp(&e), Ordering::Greater, "a : e");
        assert_eq!(a.cmp(&f), Ordering::Less, "a : f");
        assert_eq!(a.cmp(&g), Ordering::Less, "a : g");
        assert_eq!(a.cmp(&h), Ordering::Greater, "a : h");
        assert_eq!(a.cmp(&i), Ordering::Greater, "a : i");
        assert_eq!(a.cmp(&j), Ordering::Less, "a : j");
    }

    #[test]
    fn range_sort() {
        let mut data = [
            range_between(5, 7),
            range_from(4),
            range_between(2, 5),
            range_to(3),
            range_between(1, 3),
        ];
        let expected = [
            range_to(3),
            range_between(1, 3),
            range_between(2, 5),
            range_from(4),
            range_between(5, 7),
        ];
        data.sort();
        assert_eq!(data, expected);
    }

    #[test]
    fn new_range_list() {
        let data = [
            range_from(8),
            range_between(3, 3),
            range_to(2),
            range_between(5, 7),
        ];
        let expected = [
            range_to(2),
            range_between(3, 3),
            range_between(5, 7),
            range_from(8),
        ];

        let range_list = RangeList::from(data);
        assert_eq!(range_list.0, expected);
    }
}
