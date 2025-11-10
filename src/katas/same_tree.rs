use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

type Node = Rc<RefCell<TreeNode>>;

type MaybeNode = Option<Node>;

impl Solution {
    pub fn is_same_tree(p: MaybeNode, q: MaybeNode) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) if p.borrow().val == q.borrow().val => {
                return Self::is_same_tree(p.borrow_mut().left.take(), q.borrow_mut().left.take())
                    && Self::is_same_tree(p.borrow_mut().right.take(), q.borrow_mut().right.take())
            },
            _ => false
        }
    }
}

pub fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode::new(val)))
}

fn new_node_option(val: i32) -> MaybeNode {
    Option::from(new_node(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_same_tree(test_tree(), test_tree()));
    }

    #[test]
    fn test_2() {
        let tree1 = test_tree();
        let mut tree2 = test_tree();
        tree2.as_mut().unwrap().borrow_mut().left = new_node_option(12);
        assert!(!Solution::is_same_tree(tree1, tree2));
    }

    #[test]
    fn test_3() {
        let tree1 = new_node(1);
        let tree2 = new_node(1);

        tree1.borrow_mut().left = new_node_option(2);
        tree2.borrow_mut().left = new_node_option(1);
        
        tree1.borrow_mut().right = new_node_option(1);
        tree2.borrow_mut().right = new_node_option(2);
        assert!(!Solution::is_same_tree(Option::from(tree1), Option::from(tree2)));
    }

    fn test_tree() -> MaybeNode {
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
        root
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

