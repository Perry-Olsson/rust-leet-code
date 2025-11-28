pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut x = x;
        let mut odd = 1;
        let mut root = 0;
        while x > 0 {
            x -= odd;
            root += 1;
            odd +=2;
        }
        if x < 0 {
            return root - 1;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::my_sqrt(4));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::my_sqrt(5));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::my_sqrt(9));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::my_sqrt(0));
    }

    #[test]
    fn test_5() {
        assert_eq!(1, Solution::my_sqrt(1));
    }

    #[test]
    fn test_6() {
        assert_eq!(46340, Solution::my_sqrt(2147483647));
    }
}
