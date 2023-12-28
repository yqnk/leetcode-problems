// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
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

struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        if root.is_none() {
            return None;
        }
        let mut root = root.unwrap();
        let mut left = root.borrow_mut().left.take();
        let mut right = root.borrow_mut().right.take();
        root.borrow_mut().left = Self::invert_tree(right);
        root.borrow_mut().right = Self::invert_tree(left);
        Some(root)
    }
}
