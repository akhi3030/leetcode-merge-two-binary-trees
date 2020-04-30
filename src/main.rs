use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn build_tree(data: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() {
        return None;
    }

    match data[0] {
        None => None,
        Some(val) => {
            let head = Rc::new(RefCell::new(TreeNode::new(val)));
            let mut stack = vec![(head.clone(), 0)];
            loop {
                match stack.pop() {
                    None => break,
                    Some((current, index)) => {
                        let mut cur = current.borrow_mut();
                        let left_ind = index * 2 + 1;
                        if left_ind < data.len() {
                            if let Some(val) = data[left_ind] {
                                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                                cur.left = Some(left.clone());
                                stack.push((left, left_ind));
                            }
                        }
                        let right_ind = index * 2 + 2;
                        if right_ind < data.len() {
                            if let Some(val) = data[right_ind] {
                                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                                cur.right = Some(right.clone());
                                stack.push((right, right_ind));
                            }
                        }
                    }
                }
            }
            Some(head)
        }
    }
}

fn merge_trees(
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

fn main() {
    let t0 = build_tree(&[Some(1), Some(2), Some(3)]);
    let t1 = build_tree(&[Some(1), None, Some(3)]);
    let t2 = merge_trees(t0, t1);
    println!("{:#?}", t2);
}
