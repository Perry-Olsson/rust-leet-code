use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: MaybeNode) -> bool {
        return Solution::recurse(&root, &root)
    }

    fn recurse(left_side: &MaybeNode, right_side: &MaybeNode) -> bool {
        if !Solution::is_eq(&left_side, &right_side) {
            return false;
        }

        if left_side.is_none() {
            return true;
        }
        return Solution::recurse(
            &left_side.as_ref().unwrap().borrow().left,
            &right_side.as_ref().unwrap().borrow().right
        ) &&
        Solution::recurse(
            &left_side.as_ref().unwrap().borrow().right,
            &right_side.as_ref().unwrap().borrow().left
        )
    }

    fn is_eq(left_side: &MaybeNode, right_side: &MaybeNode) -> bool {
        match (left_side, right_side) {
            (Some(left), Some(right)) => {
                return left.borrow().val == right.borrow().val
            },
            (None, None) => return true,
            _ => return false
        }
    }
}
/*
 * fn recurse(left_side_node: Node, right_side_node: Node) -> bool {
 *   if left_side_node != right_side_node { return false }
 *   if left_side_node.is_none() { return true; }
 *
 *   return recurse(left_side_node.left, right_side_node.right) 
 *   && recurse(left_side_node.right, right_side_node.left)
 * }
 */

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
