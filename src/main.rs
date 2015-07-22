#![feature(step_by)]

extern crate structures;

use structures::list::List;
use structures::tree::Tree;

fn main() {
    let l: List<i32> = List::empty();
    println!("{}", l);
    let ll: List<i32> = l.cons(1);
    println!("{}", ll);
    let lll: List<i32> = List::from_slice(&[1, 2, 3]);
    println!("{}\nLength: {}", lll, lll.len());


    match ll.decons() {
        Some( (hd, tl) ) => println!("head: {}\ntail: {}", hd, tl),
        None => println!("couldn't decons.."),
    }

    for i in (10..0).step_by(-1) {
        println!("{}", i);
    }

    let _: List<&str> = List::empty().cons("hi!").cons("i AM bob!");


    let my_tree: Tree<i32> = Tree::singleton(0);
    println!("my_tree: {:?}", my_tree);

    let i = 5;
    println!("5/2 as ints is: {}", i/2);

    let mut l: List<i32> = List::Empty;
    for c in (10..0).step_by(-1) {
        l = l.cons(c);
    }
    let l = l.cons(0);
    println!("My list: {}", l);

    let t: Tree<i32> = Tree::from_list(l);
    println!("my tree: {:?}", t);

    let list: List<&str> = List::empty().cons("hi!").cons("i AM bob!");
    let tree: Tree<&str> = Tree::from_list(list);
    println!("{:?}", tree);
}
