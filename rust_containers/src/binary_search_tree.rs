use std::cmp;
use std::cmp::Ordering;
use std::vec::Vec;

pub struct Node<T: Ord> {
    value: T,
    height: i64,
    count: u32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl <T: Ord> Node<T> {
    pub const fn new(v: T) -> Node<T> {
        Node { value: v, height: 0, count: 1, left: None, right: None }
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
                Ordering::Less => p.right = insert(v, p.right.take()),
                Ordering::Greater => p.left = insert(v, p.left.take()),
                Ordering::Equal => p.count += 1,
            }
            update_height(&mut p);
            Some(balance(p))
        },
        None => {
            Some(Box::new(Node::new(v)))
        }
    }
}

fn find_left_min<T: Ord> (mut node : Box<Node<T>>, left: Box<Node<T>>) -> (Option<Box<Node<T>>>, Box<Node<T>>) {
    let (new_left, min) = find_min(left);
    node.left = new_left;
    update_height(&mut node);
    (Some(balance(node)), min)
}

fn find_min<T: Ord> (mut node: Box<Node<T>>) -> (Option<Box<Node<T>>>, Box<Node<T>>) {
    match node.left.take() {
        Some(left) => find_left_min(node, left),
        None => (node.right.take(), node)
    }
}

pub fn delete<T: Ord> (v: T, mut parent: Option<Box<Node<T>>>, size: &mut u32) -> Option<Box<Node<T>>> {
    match parent {
        Some(mut p) => {
            match p.value.cmp(&v) {
                Ordering::Less => p.right = delete(v, p.right.take(), size),
                Ordering::Greater => p.left = delete(v, p.left.take(), size),
                Ordering::Equal => {
                    *size -= p.count; // update size
                    match (p.left.take(), p.right.take()) {
                        (None, None) => return None,
                        (None, Some(r)) => return Some(r),
                        (Some(l), None) => return Some(l),
                        (Some(l), Some(r)) => {
                            let (right_sub_tree, right_min) = find_min(r);
                            p = right_min;
                            p.left = Some(l);
                            p.right = right_sub_tree;
                            update_height(&mut p);
                            return Some(balance(p));
                        }
                    }
                }
            }
            update_height(&mut p);
            Some(balance(p))
        },
        None => {
            parent
        }
    }
}

pub fn contains<T: Ord> (v: &T, parent: &Option<Box<Node<T>>>) -> bool {
    match parent {
        Some(p) => {
            match p.value.cmp(&v) {
                Ordering::Less => return contains(v, &p.right),
                Ordering::Greater => return contains(v, &p.left),
                Ordering::Equal => return true,
            }
        },
        None => {
            false
        }
    }
}

pub fn in_order_to_vec<T:Ord> (node: &Option<Box<Node<T>>>, vec: &mut Vec<T>) 
where
    T: Copy,
{
    match node {
        Some(n) => {
            in_order_to_vec(&n.left, vec);
            let mut i = n.count;
            while i > 0 {
                vec.push(n.value);
                i -= 1;
            }
            in_order_to_vec(&n.right, vec);
        },
        None => {
            return;
        },
    }
}

pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
    size: u32,
}

impl <T: Ord> BinarySearchTree<T> {
    pub const fn new() -> BinarySearchTree<T>{
        BinarySearchTree{ root: None, size: 0 }
    }

    pub fn len(&self) -> u32 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, v: T) -> bool {
        contains(&v, &self.root)
    }

    pub fn insert(&mut self, v: T) {
        self.root = insert(v, self.root.take());
        self.size += 1;
    }

    pub fn delete(&mut self, v: T) {
        self.root = delete(v, self.root.take(), &mut self.size);
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }

    pub fn to_vec(&self) -> Vec<T> 
    where
        T: Copy,
    {
        let mut vec = Vec::new();
        in_order_to_vec(&self.root, &mut vec);
        vec
    }
}
