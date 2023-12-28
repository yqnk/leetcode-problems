// Lowest Common Ancestor of a BST

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
    pub fn lowest_common_ancestor(mut root: Option<Rc<RefCell<TreeNode>>>, mut p: Option<Rc<RefCell<TreeNode>>>, mut q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            let p_val = p.as_ref()?.borrow().val;
            let q_val = q.as_ref()?.borrow().val;
            if p_val > val && q_val > val {
                root = node.right.clone();
            } else if p_val < val && q_val < val {
                root = node.left.clone();
            } else {
                return Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
        }
        None
    }
}