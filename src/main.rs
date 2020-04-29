// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn merge_trees(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match &t1 {
        None => t2,
        Some(t1) => match &t2 {
            None => Some(t1.clone()),
            Some(t2) => {
                let mut stack = vec![(t1.clone(), t2.clone())];
                loop {
                    match stack.pop() {
                        None => break,
                        Some((t1, t2)) => {
                            let mut t1 = t1.borrow_mut();
                            let t2 = t2.borrow();
                            t1.val += t2.val;
                            match &mut t1.left {
                                None => t1.left = t2.left.clone(),
                                Some(t1_left) => match &t2.left {
                                    None => (),
                                    Some(t2_left) => stack.push((t1_left.clone(), t2_left.clone())),
                                },
                            }
                            match &mut t1.right {
                                None => t1.right = t2.right.clone(),
                                Some(t1_right) => match &t2.right {
                                    None => (),
                                    Some(t2_right) => {
                                        stack.push((t1_right.clone(), t2_right.clone()))
                                    }
                                },
                            }
                        }
                    }
                }
                Some(t1.clone())
            }
        },
    }
}

fn main() {}
