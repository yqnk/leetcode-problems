// Balanced Binary Tree

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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> u32 {
        let mut r = 0;
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::height(node.left.clone());
            let right = Self::height(node.right.clone());
            r = 1 + (std::cmp::max(left, right));
        }
        r
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut result: bool = true;
        let mut stack = vec![root];
        while result && !stack.is_empty() {
            let node = stack.pop().unwrap();
            if let Some(node) = node {
                let node = node.borrow();
                let left = Self::height(node.left.clone());
                let right = Self::height(node.right.clone());
                result = (left as i32 - right as i32).abs() <= 1;
                stack.push(node.left.clone());
                stack.push(node.right.clone());
            }
        }
        result
    }
}
