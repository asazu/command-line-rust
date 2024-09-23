use super::{Range, RangeList};

pub struct RangeFilter<'a, I> {
    iter: I,
    ranges: std::slice::Iter<'a, Range>,
    last: usize,
    rest: usize,
}

impl<'a, I> RangeFilter<'a, I> {
    fn new(iter: I, range_list: &RangeList) -> RangeFilter<I> {
        RangeFilter {
            iter,
            ranges: range_list.iter(),
            last: 0,
            rest: 0,
        }
    }
}

impl<I: Iterator> RangeFilter<'_, I> {
    fn skip_to(&mut self, n: usize) {
        while self.last < n {
            self.iter.next();
            self.last += 1;
        }
    }

    fn next_range(&mut self) {
        while let Some(range) = self.ranges.next() {
            if self.last < range.end() {
                self.skip_to(range.start() - 1);
                self.rest = range.end() - self.last;
                return;
            }
        }
    }
}

impl<I: Iterator> Iterator for RangeFilter<'_, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.rest == 0 {
            self.next_range();
        }
        if self.rest == 0 {
            None
        } else {
            self.rest -= 1;
            self.last += 1;
            self.iter.next()
        }
    }
}

pub trait RangeFilterEx {
    fn range_filter(self, range_list: &RangeList) -> RangeFilter<Self>
    where
        Self: Sized;
}

impl<I: Iterator> RangeFilterEx for I {
    fn range_filter(self, range_list: &RangeList) -> RangeFilter<Self> {
        RangeFilter::new(self, range_list)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{range_between, range_from, range_to};
    use super::*;

    #[test]
    fn test_iterator() {
        let ranges = RangeList::from([
            range_between(5, 7),
            range_to(3),
            range_between(9, 9),
            range_from(12),
        ]);
        let source = 1..15;
        let expected = [1, 2, 3, 5, 6, 7, 9, 12, 13, 14];
        let actual = source.range_filter(&ranges).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_iterator_duplicate() {
        let ranges = RangeList::from([
            range_to(3),
            range_between(3, 8),
            range_between(4, 8),
            range_between(6, 10),
            range_from(6),
        ]);
        let source = 1..12;
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let actual = source.range_filter(&ranges).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_iterator_smaller_end_than_previous_range() {
        let ranges = RangeList::from([range_to(5), range_between(3, 4)]);
        let source = 1..10;
        let expected = [1, 2, 3, 4, 5];
        let actual = source.range_filter(&ranges).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
