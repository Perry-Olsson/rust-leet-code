use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

type NullableNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn inorder_traversal(root: NullableNode) -> Vec<i32> {
        let mut result = Vec::new();
        Solution::recurse(root, &mut result);
        result
    }

    pub fn recurse(node: NullableNode, result: &mut Vec<i32>) {
        if let Some(n) = node {
            Solution::recurse(n.borrow_mut().left.take(), result);
            result.push(n.borrow().val);
            Solution::recurse(n.borrow_mut().right.take(), result);
        }
    }
}

pub fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

fn new_node_option(val: i32) -> NullableNode {
    Option::from(new_node(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut root = new_node_option(1);
        root.as_mut()
            .unwrap()
            .borrow_mut().right = new_node_option(2);
        root.as_mut()
            .unwrap()
            .borrow_mut()
            .right
            .as_mut()
            .unwrap()
            .borrow_mut().left = new_node_option(3);
        let result = Solution::inorder_traversal(root);
        assert_eq!(vec![1, 3, 2], result);
    }
}

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

