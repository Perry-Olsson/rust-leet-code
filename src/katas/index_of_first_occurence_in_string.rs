pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut output_idx: i32 = -1;
        let mut ptr = 0;
        let mut idx = 0;

        let needle_len = needle.len();
        let needle_bytes = needle.into_bytes();
        let haystack_bytes = haystack.into_bytes();
        let start_char = needle_bytes[0];
        let mut start_char_idx: i32 = -1;
        while idx < haystack_bytes.len() {
            let haystack_char = haystack_bytes[idx];
            let needle_char = needle_bytes[ptr];
            if haystack_char == needle_char {
                if output_idx == -1 {
                    output_idx = idx as i32;
                } else {
                    if haystack_char == start_char && start_char_idx == -1 {
                        start_char_idx = idx as i32;
                    }
                }
                ptr += 1;
                if ptr >= needle_len {
                    return output_idx as i32;
                }
            } else {
                output_idx = -1;
                ptr = 0;
                if start_char_idx != -1 {
                    idx = start_char_idx as usize;
                    start_char_idx = -1;
                    continue;
                } else if haystack_char == start_char {
                    continue;
                }
            }
            idx += 1;
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

    #[test]
    fn test_5() {
        let haystack = String::from("mississippi");
        let needle = String::from("issip");
        let first_occurenct = Solution::str_str(haystack, needle);
        let expected = 4;
        assert_eq!(expected, first_occurenct)
    }

    #[test]
    fn test_6() {
        let haystack = String::from("mississippi");
        let needle = String::from("pi");
        let first_occurenct = Solution::str_str(haystack, needle);
        let expected = 9;
        assert_eq!(expected, first_occurenct)
    }
}
