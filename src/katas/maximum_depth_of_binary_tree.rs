use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::get_depth(root, 0)
    }

    fn get_depth(maybe_node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        match maybe_node {
            Some(node) => {
                let left_depth = Solution::get_depth(node.borrow_mut().left.take(), depth + 1);
                let right_depth = Solution::get_depth(node.borrow_mut().right.take(), depth + 1);
                left_depth.max(right_depth)
            }
            None => depth,
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
        assert_eq!(0, Solution::max_depth(None));
    }

    #[test]
    fn test_single_node() {
        let tree = node(1);
        assert_eq!(1, Solution::max_depth(tree));
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
        assert_eq!(3, Solution::max_depth(tree));
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
        assert_eq!(3, Solution::max_depth(tree));
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
        assert_eq!(3, Solution::max_depth(tree));
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
        assert_eq!(3, Solution::max_depth(tree));
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
        assert_eq!(4, Solution::max_depth(tree));
    }

    #[test]
    fn test_balanced_different_heights_ok() {
        // Tree where heights differ by exactly 1
        //      1
        //     / \
        //    2   3
        let tree = tree_with_children(
            1,
            tree_with_children(2, None, None),
            node(3),
        );
        assert_eq!(2, Solution::max_depth(tree));
    }
}

