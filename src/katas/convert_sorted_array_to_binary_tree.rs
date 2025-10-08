use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

type Node = Option<Rc<RefCell<TreeNode>>>;

fn new_node(val: i32) -> Node {
    Option::Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

fn is_less_than(node_1: &Node, node_2: &Node) -> bool {
    node_1.as_ref().unwrap().borrow().val < node_2.as_ref().unwrap().borrow().val
}

impl Solution {
    pub fn sorted_arary_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let left: i32 = 0;
        let right = (nums.len() - 1) as i32;
        let middle = (right - left) / 2;
        let mut head = new_node(nums[middle as usize]);
        Solution::recurse(&mut head, &nums, left, middle - 1);
        Solution::recurse(&mut head, &nums, middle + 1, right);
        head
    }

    fn recurse(node: &mut Node, nums: &Vec<i32>, left: i32, right: i32) {
        if left > right {
            return;
        }
        let middle = left + ((right - left) / 2);
        let new_node = new_node(nums[middle as usize]);
        if is_less_than(&new_node, &node) {
            node.as_mut().unwrap().borrow_mut().left = new_node;
            let new_head = &mut node.as_mut().unwrap().borrow_mut().left;
            Solution::recurse(new_head, nums, left, middle - 1);
            Solution::recurse(new_head, nums, middle + 1, right);
        } else {
            node.as_mut().unwrap().borrow_mut().right = new_node;
            let new_head = &mut node.as_mut().unwrap().borrow_mut().right;
            Solution::recurse(new_head, nums, left, middle - 1);
            Solution::recurse(new_head, nums, middle + 1, right);
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

    #[test]
    fn test_1() {
        let head = Solution::sorted_arary_to_bst(vec![1, 2, 3, 4]);
        println!("{:?}", head)
    }

    #[test]
    fn test_2() {
        let head = Solution::sorted_arary_to_bst(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        println!("{:?}", head)
    }
}
