pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut closest = i32::MAX;
        let mut final_sum = i32::MAX;
        for i in 0..(nums.len() - 2) {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if (target - sum).abs() < closest {
                    closest = (target - sum).abs();
                    final_sum = sum;
                }
                if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        final_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::three_sum_closest(vec![0, 0, 0], 1)); 
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::three_sum_closest(vec![0, 0, 0, 1, 0, 0, 3, 1, 0], 5)); 
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::three_sum_closest(vec![0, 0, 0, 1, 0, 0, 3, 1, 0], -5)); 
    }

    #[test]
    fn test_5() {
        assert_eq!(-4, Solution::three_sum_closest(vec![0, 0, -4, 1, 0, 0, 3, 1, 0], -5)); 
    }

    #[test]
    fn test_6() {
        assert_eq!(-4, Solution::three_sum_closest(vec![0, 0, -4, 1, 0, 100, 3, 1, -100], -5)); 
    }
}

