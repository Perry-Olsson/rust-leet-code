use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::check_balanced(root).0
    }

    fn check_balanced(maybe_node: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        if let Some(node) = maybe_node {
            let (is_left_balanced, left_height) = Solution::check_balanced(node.borrow_mut().left.take());
            let (is_right_balanced, right_height) = Solution::check_balanced(node.borrow_mut().right.take());
            let height = 1 + std::cmp::max(left_height, right_height);
            (is_left_balanced && is_right_balanced && (left_height - right_height).abs() <= 1, height)
        } else {
            return (true, 0);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn tree_with_children(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left,
            right,
        })))
    }

    #[test]
    fn test_empty_tree() {
        assert!(Solution::is_balanced(None));
    }

    #[test]
    fn test_single_node() {
        let tree = node(1);
        assert!(Solution::is_balanced(tree));
    }

    #[test]
    fn test_balanced_tree_simple() {
        // Tree: [3,9,20,null,null,15,7]
        //      3
        //     / \
        //    9  20
        //      /  \
        //     15   7
        let tree = tree_with_children(
            3,
            node(9),
            tree_with_children(20, node(15), node(7)),
        );
        assert!(Solution::is_balanced(tree));
    }

    #[test]
    fn test_balanced_tree_full() {
        // Tree: [1,2,3,4,5,6,7]
        //        1
        //       / \
        //      2   3
        //     / \ / \
        //    4  5 6  7
        let tree = tree_with_children(
            1,
            tree_with_children(2, node(4), node(5)),
            tree_with_children(3, node(6), node(7)),
        );
        assert!(Solution::is_balanced(tree));
    }

    #[test]
    fn test_unbalanced_left_heavy() {
        // Tree: [1,2,null,3]
        //    1
        //   /
        //  2
        // /
        //3
        let tree = tree_with_children(
            1,
            tree_with_children(2, node(3), None),
            None,
        );
        assert!(!Solution::is_balanced(tree));
    }

    #[test]
    fn test_unbalanced_right_heavy() {
        // Tree: [1,null,2,null,3]
        //  1
        //   \
        //    2
        //     \
        //      3
        let tree = tree_with_children(
            1,
            None,
            tree_with_children(2, None, node(3)),
        );
        assert!(!Solution::is_balanced(tree));
    }

    #[test]
    fn test_unbalanced_deep_tree() {
        // Tree: [1,2,2,3,3,null,null,4,4]
        //        1
        //       / \
        //      2   2
        //     / \
        //    3   3
        //   / \
        //  4   4
        let tree = tree_with_children(
            1,
            tree_with_children(
                2,
                tree_with_children(3, node(4), node(4)),
                node(3),
            ),
            node(2),
        );
        assert!(!Solution::is_balanced(tree));
    }

    #[test]
    fn test_balanced_different_heights_ok() {
        // Tree where heights differ by exactly 1
        //      1
        //     / \
        //    2   3
        //   /
        //  4
        let tree = tree_with_children(
            1,
            tree_with_children(2, node(4), None),
            node(3),
        );
        assert!(Solution::is_balanced(tree));
    }
}

