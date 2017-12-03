use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use BaseList::{Cons, Nil};


pub struct List<A> {
    rc: Rc<BaseList<A>>
}

enum BaseList<A> {
    Cons(A, List<A>),
    Nil
}

impl<A> Clone for List<A> {
    fn clone(&self) -> Self {
        List { rc: Rc::clone(&self.rc) }
    }
}

pub fn cons<A>(head: A, tail: List<A>) -> List<A> {
    List { rc: Rc::new(Cons(head, tail)) }
}

pub fn nil<A>() -> List<A> {
    List { rc: Rc::new(Nil) }
}

impl<A> List<A> {
    pub fn head(&self) -> &A {
        match *self.rc {
            Cons(ref h, _) => &h,
            Nil => panic!("`head` on empty List"),
        }
    }

    pub fn tail(&self) -> List<A> {
        match *self.rc {
            Cons(_, ref t) => t.clone(),
            Nil => panic!("`tail` on empty List"),
        }
    }

    pub fn head_opt(&self) -> Option<&A> {
        match *self.rc {
            Cons(ref h, _) => Some(&h),
            Nil => None,
        }
    }

    pub fn tail_opt(&self) -> Option<List<A>> {
        match *self.rc {
            Cons(_, ref t) => Some(t.clone()),
            Nil => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self.rc {
            Cons(_, _) => false,
            Nil => true,
        }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn iter(&self) -> Iter<A> {
        Iter { list: self }
    }
}

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
}
