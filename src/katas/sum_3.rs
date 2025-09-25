pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Solution::simple(nums)
    }

    fn simple(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        nums.sort();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }

        res
    }
}

// [-1,0,1,2,-1,-4] sorted: [-4, -1, -1, 0, 1, 2]
// [0, 0, 0]
