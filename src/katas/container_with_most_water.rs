pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        Solution::simple(height)
    }

    fn simple(height: Vec<i32>) -> i32 {
        let mut max_area = -1;

        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let width = (right - left) as i32;
            let height = if height[left] > height[right] {
                let h = height[right];
                right -= 1;
                h
            } else { 
                let h = height[left];
                left += 1;
                h
            };
            let area = height * width;
            if area > max_area {
                max_area = area;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = vec![1,8,6,2,5,4,8,3,7];
        let area = Solution::max_area(actual);
        assert_eq!(49, area);
    }

    #[test]
    fn test_2() {
        let actual = vec![1, 1];
        let area = Solution::max_area(actual);
        assert_eq!(1, area);
    }

    #[test]
    fn test_3() {
        let actual = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let area = Solution::max_area(actual);
        assert_eq!(7, area);
    }

    #[test]
    fn test_4() {
        let actual = vec![1, 2, 3, 4];
        let area = Solution::max_area(actual);
        assert_eq!(4, area);
    }

    #[test]
    fn test_5() {
        let actual = vec![4, 3, 2, 1];
        let area = Solution::max_area(actual);
        assert_eq!(4, area);
    }

    #[test]
    fn test_6() {
        let actual = vec![1, 20, 21, 300];
        let area = Solution::max_area(actual);
        assert_eq!(40, area);
    }

    #[test]
    fn test_7() {
        let actual = vec![300, 300, 21, 150];
        let area = Solution::max_area(actual);
        assert_eq!(150 * 3, area);
    }
}
