use crate::binaryfun::*;
use crate::queue::queue::*;

mod binaryfun;
mod queue;

fn main() {
    fn basic() {
        let mut bt = binary_tree::BinaryTree::new(10usize);

        let root = bt.get_key();
        println!("root key: {:?}", root);

        bt.set_key(11usize);
        let root = bt.get_key();
        println!("root key: {:?}", root);

        bt.insert_left_tree(2usize);
        bt.insert_right_tree(18usize);

        println!("left child: {:#?}", bt.get_left());
        println!("left child: {:#?}", bt.get_left());
        println!("right child: {:#?}", bt.get_right());
        println!("min key: {:?}", bt.min().unwrap());
        println!("max key: {:?}", bt.max().unwrap());
        println!("tree nodes: {}", bt.size());
        println!("tree leaves: {}", bt.leaf_size());
        println!("tree internals: {}", bt.none_leaf_size());
        println!("tree depth: {}", bt.depth());
        println!("tree contains '2': {}", bt.contains(&2));
    }

    let mut bt2 = binary_tree::BinaryTree::new(12usize).insert_left_tree(2usize);

    let mut q = Queue::new();

    let mut a = Queue::new();
    a.push('a');
    a.push('b');

    let (g, h) = Queue::split(a);
    println!("{:?}, {:?}", g, h);

    q.push('0');
    q.push('1');

    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 63, 64, 66, 67];

    let target = 8;

    let _found = bin_search::binary_search2(&nums, target);
    let _found = bin_search::binary_search1(&nums, target);
    //println!("nums contains {target}: {found}");
    basic();
}
