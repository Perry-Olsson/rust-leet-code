pub struct Solution {}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();
        let nums: Vec<f64> = nums.iter().map(|x| *x as f64).collect();
        let target = target as f64;
        for i in 0..(nums.len() - 3) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..(nums.len() - 2) {
                if j > (i + 1) && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut left = j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum = nums[i] + nums[j] + nums[left] + nums[right];
                    if sum == target {
                        result.push(vec![nums[i] as i32, nums[j] as i32, nums[left] as i32, nums[right] as i32]);
                    }

                    if sum < target {
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        left += 1;
                    } else {
                        while right > left && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        right -=1;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![
            vec![-2, -1, 1, 2],
            vec![-2, 0, 0, 2],
            vec![-1, 0, 0, 1]
        ], 
        Solution::four_sum(vec![1,0,-1,0,-2,2], 0)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![
            vec![2, 2, 2, 2],
        ], 
        Solution::four_sum(vec![2, 2, 2, 2, 2], 8)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![
            vec![-2, -1, 1, 2],
            vec![-1, -1, 1, 1],
        ], 
        Solution::four_sum(vec![-2,-1,-1,1,1,2,2], 0)
        );
    }

    #[test]
    fn test_4() {
        let expected: Vec<Vec<i32>> = Vec::new();
        assert_eq!(expected, 
        Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296)
        );
    }
}

