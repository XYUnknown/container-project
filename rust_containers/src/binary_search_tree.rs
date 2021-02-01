use std::cmp;
use std::cmp::Ordering;
use std::fmt;
pub struct Node<T: Ord> {
    value: T,
    height: i64,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl <T: Ord> Node<T> {
    pub const fn new(v: T) -> Node<T> {
        Node { value: v, height: 0, left: None, right: None }
    }
}

fn height<T: Ord> (node: &Option<Box<Node<T>>>) -> i64 {
    node.as_ref().map_or(-1, |n| n.height)
}

fn balance_factor<T: Ord> (node: &Option<Box<Node<T>>>) -> i64 {
    match node {
        Some(n) => {
            let lh = height(&n.left);
            let rh = height(&n.right);
            lh - rh
        },
        None => -1
    }
}

fn balance_factor_root<T: Ord> (node: &Box<Node<T>>) -> i64 {
    let lh = height(&node.left);
    let rh = height(&node.right);
    lh - rh
}

fn update_height<T: Ord> (node: &mut Node<T>) {
    node.height = cmp::max(height(&node.left), height(&node.right)) + 1
}

fn rotate_left<T: Ord> (mut parent: Box<Node<T>>) -> Box<Node<T>> {
    let mut new_parent = parent.right.take().expect("Something went wrong");
    parent.right = new_parent.left;
    update_height(&mut parent);
    new_parent.left = Some(parent);
    update_height(&mut new_parent);
    new_parent
}

fn rotate_right<T: Ord> (mut parent: Box<Node<T>>) -> Box<Node<T>> {
    let mut new_parent = parent.left.take().expect("Something went wrong");
    parent.left = new_parent.right;
    update_height(&mut parent);
    new_parent.right = Some(parent);
    update_height(&mut new_parent);
    new_parent
}

fn rotate_left_right<T: Ord> (mut parent: Box<Node<T>>) -> Box<Node<T>> {
    parent.left = Some(rotate_left(parent.left.take().expect("Something went wrong")));
    rotate_right(parent)
}

fn rotate_right_left<T: Ord> (mut parent: Box<Node<T>>) -> Box<Node<T>>{
    parent.right = Some(rotate_right(parent.right.take().expect("Something went wrong")));
    rotate_left(parent)
}

fn balance<T:Ord> (mut parent: Box<Node<T>>) -> Box<Node<T>> {
    let lbf = balance_factor(&parent.left);
    let rbf = balance_factor(&parent.right);
    let bf = balance_factor_root(&parent);
    if bf == 2 {
        if  lbf == 1 {
            return rotate_right(parent);
        } else if lbf == -1 {
            return rotate_left_right(parent);
        } 
    } else if bf == -2 {
        if rbf == -1{
            return rotate_left(parent);
        } else if rbf == 1 {
            return rotate_right_left(parent);
        } 
    }
    return parent;
}

pub fn insert<T: Ord> (v: T, mut parent: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    match parent {
        Some(mut p) => {
            match p.value.cmp(&v) {
                Ordering::Less => p.left = insert(v, p.left.take()),
                Ordering::Equal | Ordering::Greater => p.right = insert(v, p.right.take())
            }
            update_height(&mut p);
            Some(balance(p))
        },
        None => {
            Some(Box::new(Node::new(v)))
        }
    }
}

pub fn in_order_print<T:Ord + std::fmt::Display> (node: &Option<Box<Node<T>>>) {
    match node {
        Some(n) => {
            in_order_print(&n.left);
            println!("{}", n);
            in_order_print(&n.right);
        },
        None => {
            println!("empty");
        },
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(value: {})", self.value)
    }
}

pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
    size: u32,
}

impl <T: Ord> BinarySearchTree<T> 
where
    T: fmt::Display,
    {
        pub const fn new() -> BinarySearchTree<T>{
            BinarySearchTree{ root: None, size: 0 }
        }

        pub fn len(&self) -> u32 {
            self.size
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn insert(mut self, v: T) {
            self.root = insert(v, self.root);
            self.size += 1;
        }

        pub fn in_order_print(&self) {
            in_order_print(&self.root);
        }
    }
