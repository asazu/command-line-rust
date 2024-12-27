use std::cmp::Ordering;
use std::iter::Peekable;

pub enum Choice {
    Left,
    Right,
    Both,
}

pub struct Merge<L, R, F>
where
    L: Iterator,
    R: Iterator,
{
    left: Peekable<L>,
    right: Peekable<R>,
    choice: F,
}

impl<L, R, F> Iterator for Merge<L, R, F>
where
    L: Iterator,
    R: Iterator,
    F: FnMut(&L::Item, &R::Item) -> Choice,
{
    type Item = (Option<L::Item>, Option<R::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.peek(), self.right.peek()) {
            (None, None) => None,
            (Some(_), None) => Some((self.left.next(), None)),
            (None, Some(_)) => Some((None, self.right.next())),
            (Some(l), Some(r)) => match (self.choice)(l, r) {
                Choice::Left => Some((self.left.next(), None)),
                Choice::Right => Some((None, self.right.next())),
                Choice::Both => Some((self.left.next(), self.right.next())),
            },
        }
    }
}

pub fn merge_by<L, R, F>(left: L, right: R, choice: F) -> Merge<L, R, F>
where
    L: Iterator,
    R: Iterator,
    F: FnMut(&L::Item, &R::Item) -> Choice,
{
    Merge {
        left: left.peekable(),
        right: right.peekable(),
        choice,
    }
}

fn ordering_to_choice(o: Ordering) -> Choice {
    match o {
        Ordering::Less => Choice::Left,
        Ordering::Greater => Choice::Right,
        Ordering::Equal => Choice::Both,
    }
}

pub fn merge_ordered_by<L, R, F>(
    left: L,
    right: R,
    mut cmp: F,
) -> impl Iterator<Item = (Option<L::Item>, Option<R::Item>)>
where
    L: Iterator,
    R: Iterator,
    F: FnMut(&L::Item, &R::Item) -> Ordering,
{
    merge_by(left, right, move |l, r| ordering_to_choice(cmp(l, r)))
}

pub fn try_merge_by<L, R, T, S, E, F>(
    left: L,
    right: R,
    mut choice: F,
) -> impl Iterator<Item = Result<(Option<T>, Option<S>), E>>
where
    L: Iterator<Item = Result<T, E>>,
    R: Iterator<Item = Result<S, E>>,
    F: FnMut(&T, &S) -> Choice,
{
    merge_by(left, right, move |l, r| match (l, r) {
        (Err(_), _) => Choice::Left,
        (_, Err(_)) => Choice::Right,
        (Ok(ref l), Ok(ref r)) => choice(l, r),
    })
    .map(|x| match x {
        (None, None) => unreachable!(),
        (Some(Ok(l)), None) => Ok((Some(l), None)),
        (None, Some(Ok(r))) => Ok((None, Some(r))),
        (Some(Err(e)), _) => Err(e),
        (_, Some(Err(e))) => Err(e),
        (Some(Ok(l)), Some(Ok(r))) => Ok((Some(l), Some(r))),
    })
}

pub fn try_merge_ordered_by<L, R, T, E, F>(
    left: L,
    right: R,
    mut cmp: F,
) -> impl Iterator<Item = Result<(Option<T>, Option<T>), E>>
where
    L: Iterator<Item = Result<T, E>>,
    R: Iterator<Item = Result<T, E>>,
    T: Ord,
    F: FnMut(&T, &T) -> Ordering,
{
    try_merge_by(left, right, move |l, r| ordering_to_choice(cmp(l, r)))
}

#[allow(dead_code)]
pub fn merge<L, R>(left: L, right: R) -> impl Iterator<Item = (Option<L::Item>, Option<R::Item>)>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
    L::Item: Ord,
{
    merge_ordered_by(left, right, L::Item::cmp)
}

#[allow(dead_code)]
pub fn try_merge<L, R, T, E>(
    left: L,
    right: R,
) -> impl Iterator<Item = Result<(Option<T>, Option<T>), E>>
where
    L: Iterator<Item = Result<T, E>>,
    R: Iterator<Item = Result<T, E>>,
    T: Ord,
{
    try_merge_ordered_by(left, right, T::cmp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_by() {
        let left = [1, 2].into_iter();
        let right = ["a", "b"].into_iter();
        let expected = vec![(Some(1), None), (None, Some("a")), (Some(2), Some("b"))];
        let actual = merge_by(left, right, |&l, &r| match (l, r) {
            (1, _) => Choice::Left,
            (_, "a") => Choice::Right,
            _ => Choice::Both,
        })
        .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge() {
        let left = [2, 4, 6, 8].into_iter();
        let right = [3, 6, 9, 12].into_iter();
        let expected = vec![
            (Some(2), None),
            (None, Some(3)),
            (Some(4), None),
            (Some(6), Some(6)),
            (Some(8), None),
            (None, Some(9)),
            (None, Some(12)),
        ];
        let actual = merge(left, right).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_try_merge_ok() {
        let left = [Ok::<_, ()>(2), Ok(4), Ok(6), Ok(8)].into_iter();
        let right = [Ok::<_, ()>(3), Ok(6), Ok(9), Ok(12)].into_iter();
        let expected = vec![
            Ok((Some(2), None)),
            Ok((None, Some(3))),
            Ok((Some(4), None)),
            Ok((Some(6), Some(6))),
            Ok((Some(8), None)),
            Ok((None, Some(9))),
            Ok((None, Some(12))),
        ];
        let actual = try_merge(left, right).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_try_merge_err() {
        let left = [Err(2), Ok(4), Err(6)].into_iter();
        let right = [Err(3), Ok(6)].into_iter();
        let expected = vec![
            Err(2),
            Err(3),
            Ok((Some(4), None)),
            Err(6),
            Ok((None, Some(6))),
        ];
        let actual = try_merge(left, right).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
