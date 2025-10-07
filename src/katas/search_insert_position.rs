pub struct Solution {}

impl Solution {

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::two(nums, target)
    }

    fn two(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;
        while left <= right {
            let mid = left + ((right - left) / 2);
            let val = nums[mid as usize];
            if target == val {
                left = mid;
                break;
            } else if target > val {
                left = mid + 1;
            } else {
                right = mid - 1
            }
        }
        left as i32
    }

    fn one(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + ((right - left) / 2);
            let val = nums[mid];
            if val == target {
                return mid as i32;
            }
            if Solution::can_no_longer_traverse(left, mid) {
                if Solution::is_target_largest_value(target, val, nums[right]) {
                    return nums.len() as i32
                } else if Solution::is_target_smallest_value(target, nums[left]) {
                    return 0;
                } else {
                    return (mid + 1) as i32;
                }
            }

            if target > val {
                left = mid;
            } else {
                right = mid;
            }
        }
        panic!("Shouldn't hit this")
    }

    fn can_no_longer_traverse(left: usize, mid: usize) -> bool {
        left == mid
    }

    fn is_target_largest_value(target: i32, val: i32, right_val: i32) -> bool {
        target > val && target > right_val
    }

    fn is_target_smallest_value(target: i32, left_val: i32) -> bool {
        target <= left_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1,3,5,6];
        let target = 5;
        let output = Solution::search_insert(nums, target);
        assert_eq!(2, output);
    }

    #[test]
    fn test_2() {
        let nums = vec![1,3,5,6];
        let target = 2;
        let output = Solution::search_insert(nums, target);
        assert_eq!(1, output);
    }

    #[test]
    fn test_3() {
        let nums = vec![1,3,5,6];
        let target = 7;
        let output = Solution::search_insert(nums, target);
        assert_eq!(4, output);
    }

    #[test]
    fn test_4() {
        let nums = vec![3];
        let target = 3;
        let output = Solution::search_insert(nums, target);
        assert_eq!(0, output);
    }

    #[test]
    fn test_5() {
        let nums = vec![3];
        let target = 2;
        let output = Solution::search_insert(nums, target);
        assert_eq!(0, output);
    }

    #[test]
    fn test_6() {
        let nums = vec![3];
        let target = 4;
        let output = Solution::search_insert(nums, target);
        assert_eq!(1, output);
    }

    #[test]
    fn test_7() {
        let nums = vec![1,3,5,6];
        let target = -11;
        let output = Solution::search_insert(nums, target);
        assert_eq!(0, output);
    }

    #[test]
    fn test_8() {
        let nums = vec![1,3,5,7,9,11,13];
        let target = 5;
        let output = Solution::search_insert(nums, target);
        assert_eq!(2, output);
    }

    #[test]
    fn test_9() {
        let nums = vec![1,3,5,7,9,11,13];
        let target = 4;
        let output = Solution::search_insert(nums, target);
        assert_eq!(2, output);
    }

    #[test]
    fn test_10() {
        let nums = vec![1,3,5,7,9,11,13];
        let target = 6;
        let output = Solution::search_insert(nums, target);
        assert_eq!(3, output);
    }

    #[test]
    fn test_11() {
        let nums = vec![1,3,5,7,9,11,13];
        let target = 13;
        let output = Solution::search_insert(nums, target);
        assert_eq!(6, output);
    }

    #[test]
    fn test_12() {
        let nums = vec![1,3,5,7,9,11,13];
        let target = 12;
        let output = Solution::search_insert(nums, target);
        assert_eq!(6, output);
    }

    #[test]
    fn test_13() {
        let nums = vec![1,3,5,7,9,11,13];
        let target = 10;
        let output = Solution::search_insert(nums, target);
        assert_eq!(5, output);
    }
}
