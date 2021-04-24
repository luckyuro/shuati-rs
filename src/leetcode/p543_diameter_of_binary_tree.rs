struct Solution;

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

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        tree_diameter_calculator(&root).1
    }
}

fn tree_diameter_calculator(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    match node {
        None =>  (-1, 0),
        Some(x) => {
            let (left_depth, left_max) = tree_diameter_calculator(&x.borrow().left);
            let (right_depth, right_max) = tree_diameter_calculator(&x.borrow().right);
            let child_max = std::cmp::max(left_max, right_max);
            let depth = std::cmp::max(left_depth, right_depth);
            (depth + 1, std::cmp::max(left_depth+right_depth+2, child_max))
        }
    }
}