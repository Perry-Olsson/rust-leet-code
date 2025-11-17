pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let st = x.to_string();
        if st.starts_with("-") {
            return false;
        }

        let mut chars = st.chars();
        while let (Some(left), Some(right)) = (chars.next(), chars.next_back()) {
            if left != right {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_palindrome(121))
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_palindrome(-121))
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_palindrome(10))
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_palindrome(1001))
    }
}
