//! Collections that hold at least one element, so "nothing chosen" cannot be written where it
//! would mean something else.

use std::collections::BTreeSet;

/// A list of at least one element, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmpty<T>(Vec<T>);

#[expect(clippy::len_without_is_empty, reason = "never empty")]
impl<T> NonEmpty<T> {
    /// The list, or `None` if it is empty.
    pub fn new(items: Vec<T>) -> Option<NonEmpty<T>> {
        (!items.is_empty()).then_some(NonEmpty(items))
    }

    pub fn one(item: T) -> NonEmpty<T> {
        NonEmpty(vec![item])
    }

    pub fn first(&self) -> &T {
        &self.0[0]
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    /// Each element mapped, still at least one.
    pub fn map<U>(self, f: impl FnMut(T) -> U) -> NonEmpty<U> {
        NonEmpty(self.0.into_iter().map(f).collect())
    }

    /// Each element and its index mapped, still at least one, or the first failure.
    pub fn try_map_enumerated<U, E>(
        &self,
        mut f: impl FnMut(usize, &T) -> Result<U, E>,
    ) -> Result<NonEmpty<U>, E> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, x)| f(i, x))
            .collect::<Result<Vec<U>, E>>()
            .map(NonEmpty)
    }
}

impl<T> IntoIterator for NonEmpty<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a NonEmpty<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// A set of at least one element, in ascending order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NonEmptySet<T: Ord>(BTreeSet<T>);

#[expect(clippy::len_without_is_empty, reason = "never empty")]
impl<T: Ord> NonEmptySet<T> {
    /// The set, or `None` if it is empty.
    pub fn new(items: BTreeSet<T>) -> Option<NonEmptySet<T>> {
        (!items.is_empty()).then_some(NonEmptySet(items))
    }

    /// The set of what `items` yields, or `None` if it yields nothing.
    pub fn collect(items: impl IntoIterator<Item = T>) -> Option<NonEmptySet<T>> {
        NonEmptySet::new(items.into_iter().collect())
    }

    pub fn one(item: T) -> NonEmptySet<T> {
        NonEmptySet(BTreeSet::from([item]))
    }

    pub fn contains(&self, item: &T) -> bool {
        self.0.contains(item)
    }

    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, T> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_set(&self) -> &BTreeSet<T> {
        &self.0
    }

    pub fn insert(&mut self, item: T) -> bool {
        self.0.insert(item)
    }

    /// The set without `item`, or `None` if that would leave it empty.
    pub fn without(mut self, item: &T) -> Option<NonEmptySet<T>> {
        self.0.remove(item);
        NonEmptySet::new(self.0)
    }

    pub fn into_set(self) -> BTreeSet<T> {
        self.0
    }
}

impl<T: Ord> IntoIterator for NonEmptySet<T> {
    type Item = T;
    type IntoIter = std::collections::btree_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: Ord> IntoIterator for &'a NonEmptySet<T> {
    type Item = &'a T;
    type IntoIter = std::collections::btree_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_is_not_a_non_empty_collection() {
        assert_eq!(NonEmpty::<u8>::new(Vec::new()), None);
        assert_eq!(NonEmptySet::<u8>::new(BTreeSet::new()), None);
        assert_eq!(NonEmptySet::<u8>::collect([]), None);
    }

    #[test]
    fn removing_the_last_element_leaves_nothing() {
        let s = NonEmptySet::one(3u8);
        assert_eq!(s.clone().without(&4), Some(s.clone()));
        assert_eq!(s.without(&3), None);
    }

    #[test]
    fn a_non_empty_list_keeps_its_order() {
        let l = NonEmpty::new(vec![3, 1, 2]).expect("three");
        assert_eq!(l.first(), &3);
        assert_eq!(l.map(|x| x * 2).into_vec(), vec![6, 2, 4]);
    }
}
