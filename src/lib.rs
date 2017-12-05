//! An immutable [cons list](https://en.wikipedia.org/wiki/Cons) designed
//! to be easily and cheaply sharable through cloning.

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::rc::Rc;
use BaseList::{Cons, Nil};


/// An immutable cons list.
///
/// # Examples
///
/// A list can be created using [`nil`](fn.nil.html) and [`cons`](fn.cons.html).
///
/// ```rust
/// // A list containing `1, 2, 3`
/// let list = cons(1, cons(2, cons(3, nil())));
/// ```
///
/// Lists can be cheaply shared using `.clone()`.
///
/// ```rust
/// let list1 = cons(1, cons(2, cons(3, nil())));
/// let list2 = list1.clone();
/// ```
pub struct List<A> {
    rc: Rc<BaseList<A>>
}

enum BaseList<A> {
    Cons(A, List<A>),
    Nil
}

impl<A> Clone for List<A> {
    /// Clones the list by cloning a reference counted pointer to
    /// the list's contents; this operation is very cheap.
    fn clone(&self) -> Self {
        List { rc: Rc::clone(&self.rc) }
    }
}

/// Prepends the specified element at the head of the specified list.
pub fn cons<A>(head: A, tail: List<A>) -> List<A> {
    List { rc: Rc::new(Cons(head, tail)) }
}

/// Returns the empty list.
pub fn nil<A>() -> List<A> {
    List { rc: Rc::new(Nil) }
}

impl<A> List<A> {
    /// Returns the first element of the list.
    ///
    /// # Panics
    ///
    /// Panics if the list is empty.
    pub fn head(&self) -> &A {
        match *self.rc {
            Cons(ref h, _) => &h,
            Nil => panic!("`head` on empty List"),
        }
    }

    /// Returns a list containing all elements except the first.
    ///
    /// # Panics
    ///
    /// Panics if the list is empty.
    pub fn tail(&self) -> List<A> {
        match *self.rc {
            Cons(_, ref t) => t.clone(),
            Nil => panic!("`tail` on empty List"),
        }
    }

    /// Returns the first element of the list, or `None` if
    /// this list is empty.
    pub fn head_opt(&self) -> Option<&A> {
        match *self.rc {
            Cons(ref h, _) => Some(&h),
            Nil => None,
        }
    }

    /// Returns a list containing all elements except the first,
    /// or `None` if this list is empty.
    pub fn tail_opt(&self) -> Option<List<A>> {
        match *self.rc {
            Cons(_, ref t) => Some(t.clone()),
            Nil => None,
        }
    }

    /// Tests whether this list is empty.
    pub fn is_empty(&self) -> bool {
        match *self.rc {
            Cons(_, _) => false,
            Nil => true,
        }
    }

    /// Returns the length of this list.
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Returns an iterator over the elements of this list.
    pub fn iter(&self) -> Iter<A> {
        Iter { list: self }
    }

    /// Returns a list with the elements in reverse order.
    pub fn rev(&self) -> List<A> where A: Clone {
        let mut list = nil();
        let mut rest = self;

        while let Cons(ref h, ref t) = *rest.rc {
            list = cons(h.clone(), list);
            rest = t;
        }

        list
    }

    /// Creates a new list from a `DoubleEndedIterator`.
    pub fn from_double_ended_iter<I: DoubleEndedIterator<Item=A>>(iter: I) -> List<A> {
        let mut list = nil();
        for elem in iter.rev() {
            list = cons(elem, list);
        }
        list
    }
}

/// An iterator over a [list](struct.List.html).
pub struct Iter<'a, A: 'a> {
    list: &'a List<A>
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        match *self.list.rc {
            Cons(ref h, ref t) => {
                self.list = t;
                Some(h)
            }
            Nil => None,
        }
    }
}

impl<'a, A: 'a> IntoIterator for &'a List<A> {
    type Item = &'a A;
    type IntoIter = Iter<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let elems: Vec<A> = iter.into_iter().collect();
        List::from_double_ended_iter(elems.into_iter())
    }
}

impl<A: Display> Display for List<A> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut list = self;

        while let Cons(ref h, ref t) = *list.rc {
            write!(f, "{} :: ", *h)?;
            list = t;
        }
        write!(f, "Nil")
    }
}

impl<A: Debug> Debug for List<A> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut list = self;

        while let Cons(ref h, ref t) = *list.rc {
            write!(f, "{:?} :: ", *h)?;
            list = t;
        }
        write!(f, "Nil")
    }
}

impl<A: PartialEq> PartialEq for List<A> {
    fn eq(&self, other: &List<A>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<A: Eq> Eq for List<A> {}

impl<A: Hash> Hash for List<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|elem| elem.hash(state));
    }
}

impl<A: PartialOrd> PartialOrd for List<A> {
    fn partial_cmp(&self, other: &List<A>) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<A: Ord> Ord for List<A> {
    fn cmp(&self, other: &List<A>) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<A> Default for List<A> {
    fn default() -> Self {
        nil()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_nil() {
        let nil = nil::<i32>();
        assert!(nil.is_empty());
        assert_eq!(nil.len(), 0);
        assert!(nil.head_opt().is_none());
        assert!(nil.tail_opt().is_none());
    }

    #[test]
    #[should_panic]
    fn test_nil_panic_head() {
        nil::<i32>().head();
    }

    #[test]
    #[should_panic]
    fn test_nil_panic_tail() {
        nil::<i32>().tail();
    }

    #[test]
    fn test_cons() {
        let nil = nil();
        let a = cons(3, cons(2, cons(1, nil.clone())));
        let b = cons(4, a.clone());

        assert_eq!(a.len(), 3);
        assert_eq!(b.len(), 4);

        let list = b;
        assert_eq!(*list.head(), 4);
        let list = list.tail();
        assert_eq!(*list.head(), 3);
        let list = list.tail();
        assert_eq!(*list.head(), 2);
        let list = list.tail();
        assert_eq!(*list.head(), 1);
        let list = list.tail();
        assert!(list.is_empty());

        let list = cons(0, nil.clone());
        assert_eq!(*list.head_opt().unwrap(), 0);
        let list = list.tail_opt().unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn test_rev() {
        let nil = nil();
        let a = cons(3, cons(2, cons(1, nil.clone())));

        assert_eq!(a.rev(), cons(1, cons(2, cons(3, nil.clone()))));
        assert_eq!(nil.rev(), nil);
    }

    #[test]
    fn test_to_from_iterator() {
        let nil = nil();
        let a = cons(3, cons(2, cons(1, nil.clone())));

        assert_eq!(a.iter().map(|i| *i).collect::<List<i32>>(), a);
        assert_eq!(vec![3, 2, 1].into_iter().collect::<List<i32>>(), a);
        assert_eq!(List::from_double_ended_iter(vec![3, 2, 1].into_iter()), a);
    }

    #[test]
    fn test_fmt() {
        assert_eq!(format!("{}", cons(3, cons(2, cons(1, nil())))),
                   "3 :: 2 :: 1 :: Nil");

        assert_eq!(format!("{:?}", cons(3, cons(2, cons(1, nil())))),
                   "3 :: 2 :: 1 :: Nil");
    }

    #[test]
    fn test_eq() {
        let nil = nil();
        let a = cons("a", nil.clone());
        let b = cons("b", nil.clone());

        // Test basic properties
        assert_eq!(a, cons("a", nil.clone()));
        assert_eq!(b, cons("b", nil.clone()));
        assert_ne!(a, b);

        // reflexive
        assert_eq!(a, a);
        assert_eq!(b, b);
        assert_eq!(b, b.clone());

        // symmetric
        assert_eq!(a, a.clone());
        assert_eq!(a.clone(), a);
        assert_eq!(a, cons("a", nil.clone()));
        assert_eq!(cons("a", nil.clone()), a);

        // transitive
        let c = cons("b", nil.clone());
        let d = cons("b", nil.clone());
        assert_eq!(b, c);
        assert_eq!(c, d);
        assert_eq!(b, d);

        // hashing
        let e = cons("a", nil.clone());
        let mut hasher = DefaultHasher::new();
        assert_eq!(a, e);
        assert_eq!(a.hash(&mut hasher), e.hash(&mut hasher));
    }

    #[test]
    fn test_cmp() {
        let nil = nil();
        let a = cons(1, nil.clone());
        let b = cons(2, nil.clone());
        let c = cons(1, cons(2, nil.clone()));

        assert_eq!(a.cmp(&cons(1, nil.clone())), Ordering::Equal);
        assert_eq!(nil.cmp(&nil), Ordering::Equal);
        assert!(a < b);
        assert!(a < c);
        assert!(a > nil);
        assert!(b > a);
        assert!(b > c);
        assert!(nil < c);
    }

    #[test]
    fn test_partial_cmp() {
        let nil = nil();
        let a = cons(1, nil.clone());
        let b = cons(2, nil.clone());
        let c = cons(1, cons(2, nil.clone()));

        assert_eq!(a.partial_cmp(&cons(1, nil.clone())), Some(Ordering::Equal));
        assert_eq!(nil.partial_cmp(&nil), Some(Ordering::Equal));
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(a.partial_cmp(&nil), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&c), Some(Ordering::Greater));
        assert_eq!(nil.partial_cmp(&c), Some(Ordering::Less));
    }

    #[test]
    fn test_default() {
        assert_eq!(List::<i32>::default(), nil())
    }
}
