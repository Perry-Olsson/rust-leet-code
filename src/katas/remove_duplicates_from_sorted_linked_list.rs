pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut cur_node = head.as_mut().unwrap();
        while let Some(next_node) = cur_node.next.as_mut() {
            if cur_node.val == next_node.val {
                cur_node.next = next_node.next.take();
            } else {
                cur_node = cur_node.next.as_mut().unwrap();
            }
        }
        head
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        let filtered = Solution::delete_duplicates(head);
        assert_ll_eq(filtered, vec![1, 2]);
    }

    fn assert_ll_eq(head: Option<Box<ListNode>>, values: Vec<i32>) {
        if values.len() > 0 {
            assert!(head != None);
        }
        let mut cur = head;
        let mut idx = 0;
        while let Some(node) = cur {
            assert!(idx < values.len());
            assert_eq!(values[idx], node.val);
            cur = node.next;
            idx += 1;
        }
    }
}

