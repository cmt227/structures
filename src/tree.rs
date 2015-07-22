use list::List;

#[derive(Debug, Clone)]
pub enum Tree<T> {
    Leaf (T),
    Node (T, Box<Tree<T>>, Box<Tree<T>>),
    Null,
}

impl<T: Copy> Tree<T> {
    pub fn singleton(val: T) -> Tree<T> {
        Tree::Leaf(val)
    }

    pub fn from_list(list: List<T>) -> Tree<T> {
        match list.len() {
            0 => Tree::Null,
            1 => {
                let one = list.nth(0).unwrap();
                Tree::Leaf(one)
            },
            2 => {
                let one = list.nth(0).unwrap();
                let two = list.nth(1).unwrap();
                Tree::Node(one, Box::new(Tree::Leaf(two)), Box::new(Tree::Null))
            },
            l => {
                let one = list.nth(0).unwrap();
                let r1: List<T> = list.range(1, ((l + 1) / 2)).unwrap();
                let r2: List<T> = list.range(((l + 1) / 2), l).unwrap();
                Tree::Node(one, Box::new(Tree::from_list(r1)), Box::new(Tree::from_list(r2)))
            },
        }
    }

}

#[cfg(test)]
mod test {
    use super::Tree;

    #[test]
    fn test_singleton() {
        match Tree::singleton(0) {
            Tree::Leaf(0) => {},
            _ => panic!(),
        }
    }


}
