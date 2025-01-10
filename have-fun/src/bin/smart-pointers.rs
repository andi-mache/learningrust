// box pointers it is a type of smart pointer that allows you to manually allocate memory on the
// heap
#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

/*
*
*Rc and Arc are reference counted pointers that allow multiple ownership of a memory allocation.
*Similar to Box, they allocate memory in the heap, but what differentiates them from
*Box is that they also include a reference count.

*
*/

use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn rc_and_arc() {
    // A chain of nodes
    let node1 = Rc::new(Node {
        value: 1,
        next: None,
    });
    let node2 = Rc::new(Node {
        value: 2,
        next: Some(Rc::clone(&node1)),
    });
    let node3 = Rc::new(Node {
        value: 3,
        next: Some(Rc::clone(&node2)),
    });

    // Multiple owners of node2
    let another_ref_to_node2 = Rc::clone(&node2);

    println!("Node 3: {:?}", node3);
    println!("Another reference to Node 2: {:?}", another_ref_to_node2);
}

fn main() {
    let point = Box::new(Point { x: 0.0, y: 0.0 });
    println!("{:?}", point);
}
