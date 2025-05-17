use std::iter::Peekable;

pub struct Uniq<I, F> {
    iter: I,
    eq: F,
}

impl<I, F> Iterator for Uniq<Peekable<I>, F>
where
    I: Iterator,
    F: Fn(&I::Item, &I::Item) -> bool,
{
    type Item = (usize, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.iter.next()?;
        let mut count = 1;
        while let Some(x) = self.iter.peek() {
            if !((self.eq)(x, &next_item)) {
                break;
            }
            self.iter.next();
            count += 1;
        }
        Some((count, next_item))
    }
}

type _Eq<T> = fn(&T, &T) -> bool;

#[allow(dead_code)]
pub trait Unique: Iterator {
    fn uniq(self) -> Uniq<Peekable<Self>, _Eq<Self::Item>>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        Uniq {
            iter: self.peekable(),
            eq: Self::Item::eq,
        }
    }

    fn uniq_by<F>(self, eq: F) -> Uniq<Peekable<Self>, F>
    where
        Self: Sized,
        F: Fn(&Self::Item, &Self::Item) -> bool,
    {
        Uniq {
            iter: self.peekable(),
            eq,
        }
    }
}

impl<I> Unique for I where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniq_iter() {
        let v = vec![1, 2, 2, 3];
        let mut iter = v.into_iter().uniq();

        assert_eq! {Some((1, 1)), iter.next()};
        assert_eq! {Some((2, 2)), iter.next()};
        assert_eq! {Some((1, 3)), iter.next()};
        assert_eq! {None, iter.next()};
    }

    #[test]
    fn uniq_iter_result() {
        let v = vec![1, 2, 2, 3, 4];
        let mut iter = v
            .into_iter()
            .map(|x| if x == 3 { Err(()) } else { Ok(x) })
            .uniq_by(|x, y| x.as_ref().ok() == y.as_ref().ok());

        assert_eq! {Some((1, Ok(1))), iter.next()};
        assert_eq! {Some((2, Ok(2))), iter.next()};
        assert_eq! {Some((1, Err(()))), iter.next()};
        assert_eq! {Some((1, Ok(4))), iter.next()};
        assert_eq! {None, iter.next()};
    }
}
