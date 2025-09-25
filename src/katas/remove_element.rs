pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        Solution::optimized(nums, val)
    }

    fn optimized(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut elements_removed = 0;
        let mut shuffle_idx = 0;

        let mut i = 0;
        while i < nums.len() {
            let num = nums[i];
            if num == val {
                elements_removed += 1;
            } else {
                nums[shuffle_idx] = num;
                shuffle_idx += 1;
            }
            i += 1;
        };

        nums.len() as i32 - elements_removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut actual = vec![3, 2, 2, 3];
        let expected = vec![2, 2];
        let new_array_len = Solution::remove_element(&mut actual, 3);
        assert_eq!(2, new_array_len);
        assert_equality(expected, actual);
    }

    #[test]
    fn test_2() {
        let mut actual = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let expected = vec![0, 1, 4, 0, 3];
        let new_array_len = Solution::remove_element(&mut actual, 2);
        assert_eq!(5, new_array_len);
        assert_equality(expected, actual);
    }

    #[test]
    fn test_3() {
        let mut actual = vec![];
        let expected = vec![];
        let new_array_len = Solution::remove_element(&mut actual, 3);
        assert_eq!(0, new_array_len);
        assert_equality(expected, actual);
    }

    #[test]
    fn test_4() {
        let mut actual = vec![4];
        let expected = vec![4];
        let new_array_len = Solution::remove_element(&mut actual, 2);
        assert_eq!(1, new_array_len);
        assert_equality(expected, actual);
    }

    #[test]
    fn test_5() {
        let mut actual = vec![1, 1, 1];
        let expected = vec![];
        let new_array_len = Solution::remove_element(&mut actual, 1);
        assert_eq!(0, new_array_len);
        assert_equality(expected, actual);
    }

    #[test]
    fn test_6() {
        let mut actual = vec![1, 1, 1, 3, 1, 1, 1];
        let expected = vec![3];
        let new_array_len = Solution::remove_element(&mut actual, 1);
        assert_eq!(1, new_array_len);
        assert_equality(expected, actual);
    }

    #[test]
    fn test_7() {
        let mut actual = vec![5, 3, 11];
        let expected = vec![5, 3, 11];
        let new_array_len = Solution::remove_element(&mut actual, 56);
        assert_eq!(3, new_array_len);
        assert_equality(expected, actual);
    }

    #[test]
    fn test_8() {
        let mut actual = vec![4];
        let expected = vec![];
        let new_array_len = Solution::remove_element(&mut actual, 4);
        assert_eq!(0, new_array_len);
        assert_equality(expected, actual);
    }

    fn assert_equality(mut expected: Vec<i32>, mut actual: Vec<i32>) -> bool {
        expected.sort();
        actual.sort();
        let mut idx = 0;
        while idx < expected.len() {
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
