pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut output_idx: i32 = -1;
        let mut char_ptr = 0;
        for (i, haystack_char) in haystack.chars().enumerate() {
            let needle_char = needle.chars().nth(char_ptr).unwrap();
            if haystack_char == needle_char {
                if output_idx == -1 {
                    output_idx = i as i32;
                }
                char_ptr += 1;
                if char_ptr >= needle.len() {
                    return output_idx as i32;
                }
            } else {
                output_idx = -1;
                char_ptr = 0;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let first_occurenct = Solution::str_str(haystack, needle);
        let expected = 0;
        assert_eq!(expected, first_occurenct)
    }

    #[test]
    fn test_2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let first_occurenct = Solution::str_str(haystack, needle);
        let expected = -1;
        assert_eq!(expected, first_occurenct)
    }

    #[test]
    fn test_3() {
        let haystack = String::from("a");
        let needle = String::from("a");
        let first_occurenct = Solution::str_str(haystack, needle);
        let expected = 0;
        assert_eq!(expected, first_occurenct)
    }

    #[test]
    fn test_4() {
        let haystack = String::from("abdhelloabcdabc");
        let needle = String::from("abc");
        let first_occurenct = Solution::str_str(haystack, needle);
        let expected = 8;
        assert_eq!(expected, first_occurenct)
    }
}
