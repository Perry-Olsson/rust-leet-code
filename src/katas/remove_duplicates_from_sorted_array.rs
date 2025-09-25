pub struct Solution {
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut cur_idx = 1;
        let mut cur_val = nums[0];
        let mut move_to_idx = 1;
        let mut unique_elements = 1;

        while cur_idx < nums.len() {
            if cur_val != nums[cur_idx] {
                cur_val = nums[cur_idx];
                nums[move_to_idx] = cur_val;
                move_to_idx += 1;
                unique_elements += 1;
            }
            cur_idx += 1;
        };

        unique_elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut actual = vec![1, 2, 2, 3];
        let removed_elements = Solution::remove_duplicates(&mut actual);
        let expected = vec![1, 2, 3];
        assert!(assert_equality(expected, actual, removed_elements));
        assert_eq!(3, removed_elements)
    }

    #[test]
    fn test_2() {
        let mut actual = vec![1, 2, 2, 3, 4, 4, 5];
        let removed_elements = Solution::remove_duplicates(&mut actual);
        let expected = vec![1, 2, 3, 4, 5];
        assert!(assert_equality(expected, actual, removed_elements));
        assert_eq!(5, removed_elements)
    }

    #[test]
    fn test_3() {
        let mut actual = vec![1, 2, 3, 4, 5];
        let removed_elements = Solution::remove_duplicates(&mut actual);
        let expected = vec![1, 2, 3, 4, 5];
        assert!(assert_equality(expected, actual, removed_elements));
        assert_eq!(5, removed_elements)
    }

    #[test]
    fn test_4() {
        let mut actual = vec![1];
        let removed_elements = Solution::remove_duplicates(&mut actual);
        let expected = vec![1];
        assert!(assert_equality(expected, actual, removed_elements));
        assert_eq!(1, removed_elements)
    }

    #[test]
    fn test_5() {
        let mut actual = vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3];
        let removed_elements = Solution::remove_duplicates(&mut actual);
        let expected = vec![3];
        assert!(assert_equality(expected, actual, removed_elements));
        assert_eq!(1, removed_elements)
    }

    #[test]
    fn test_6() {
        let mut actual = vec![];
        let removed_elements = Solution::remove_duplicates(&mut actual);
        let expected = vec![];
        assert!(assert_equality(expected, actual, removed_elements));
        assert_eq!(0, removed_elements)
    }

    #[test]
    fn test_7() {
        let mut actual = vec![1, 1, 2];
        let removed_elements = Solution::remove_duplicates(&mut actual);
        let expected = vec![1, 2];
        println!("{:?}", actual);
        assert!(assert_equality(expected, actual, removed_elements));
        assert_eq!(2, removed_elements)
    }

    fn assert_equality(expected: Vec<i32>, actual: Vec<i32>, removed_elements: i32) -> bool {
        let mut idx = 0;
        while idx < removed_elements as usize {
            if expected[idx] != actual[idx] {
                println!("Test Failed!");
                println!("expected: {:?}", expected);
                println!("actual: {:?}", actual);
                return false;
            }
            idx += 1;
        }
        return true;
    }
}
