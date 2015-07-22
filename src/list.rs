use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum List<T> {
    Empty,
    Cons (T, Box<List<T>>),
}

impl<T: Copy> List<T> {

    pub fn empty() -> List<T> {
        List::Empty
    }

    pub fn from_slice(sl: &[T]) -> List<T> {
        let l = sl.len();
        if l == 0 {
            List::Empty
        } else if l == 1 {
            List::Cons(sl[0], Box::new(List::Empty))
        } else {
            List::Cons(sl[0], Box::new(Self::from_slice(&sl[1..l])))
        }
    }
    
    pub fn cons(&self, val: T) -> List<T> {
        List::Cons(val, Box::new(self.clone()))
    }
    
    pub fn decons(&self) -> Option<(T, List<T>)> {
        match self.clone() {
            List::Cons(a, b) => Some( (a, *b) ),
            _ => None,
        }
    }

    pub fn len(&self) -> i32 {
        match self.decons() {
            Some( (_, tl) ) => 1 + tl.len(),
            None => 0,
        }
    }

    pub fn nth(&self, n: i32) -> Option<T> {
        match self.decons() {
            Some( (hd, tl) ) => {
                if n == 0 {
                    Some(hd)
                } else {
                    tl.nth(n - 1)
                }
            },
            None => None,
        }
    }

    // Returns the sub-list of self including indices [start, end)
    pub fn range(&self, start: i32, end: i32) -> Option<List<T>> {
        let mut res = List::Empty;
        for i in ((end - 1)..start).step_by(-1) {
            if let Some(elem) = self.nth(i) {
                res = res.cons(elem);
            } else {
                return None;
            }
        }
        if let Some(elem) = self.nth(start) {
            Some(res.cons(elem))
        } else {
            None
        }
    }

}

impl<T: fmt::Display + Copy> fmt::Display for List<T> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list: List<T> = self.clone();
        let _ = write!(f, "[");
        while let Some( (hd, tl) ) = list.decons() {
            if let List::Empty = tl {
                let _ = write!(f, "{}", hd);
            } else {
                let _ = write!(f, "{}, ", hd);
            }
            list = tl;
        }
        write!(f, "]")
    }

}

#[cfg(test)]
mod test {
    use super::List::*;

    fn get_list() -> super::List<i32> {
        let mut l = Empty;
        for i in 0..30 {
            l = l.cons(i);
        }
        l
    }

    #[test]
    fn test_empty() {
        let l: super::List<bool> = Empty;
        assert_eq!(l, Empty);
        assert_eq!(l.len(), 0);
        assert_eq!(l.cons(true), Cons(true, Box::new(Empty)));
        assert_eq!(l.decons(), None);
    }

    #[test]
    fn test_len() {
        let l = get_list();
        assert_eq!(l.len(), 30);
    }

    #[test]
    fn test_nth() {
        let l = get_list();
        assert_eq!(l.nth(0), Some(29));
        assert_eq!(l.nth(11), Some(18));
        assert_eq!(l.nth(30), None);
        assert_eq!(l.nth(100), None);
    }

    #[test]
    fn test_range() {
        let l = get_list();
        let sub1 = Empty.cons(28).cons(29);
        let sub2 = Empty.cons(0).cons(1);
        assert_eq!(l.range(0, 2), Some(sub1));
        assert_eq!(l.range(28, l.len()), Some(sub2));
        assert_eq!(l.range(2, 33), None);
        assert_eq!(l.range(-1, 0), None);
    }

}
