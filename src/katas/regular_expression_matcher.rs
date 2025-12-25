pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut pattern = p.chars().peekable();
        let mut input = s.chars();
        let char = pattern.next();
        if let Some(c) = char {
            let next = pattern.peek();
            match next {
                Some(nc) if *nc == '*' => {
                    if *nc == '*' {
                        println!("matching zero or more of {}, on {}", c, input.next().expect("End of string"));
                    }
                }
                _ => {
                    if c == '.' {
                        println!("Matching any char on {}", input.next().expect("End of string"));
                    } else {
                        println!("Matching char {}, on {}", c, input.next().expect("End of string"));
                    }
                }
            }
        }
        true
    }
}

/* trait Matcher {
    fn match_str(&self, s: &str) -> usize;
} */
//
// struct CharMatcher {}
// struct AnyMatcher {}
// struct ZeroOrMoreMatcher {}
//
// 1. get_matcher -> (Matcher, index)
// 2. Matcher.match_str(s[cur_idx..]) -> nxt_idx
// 3. 

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_exact_match() {
        assert_eq!(Solution::is_match("abc".to_string(), "abc".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "a".to_string()), true);
        assert_eq!(Solution::is_match("test".to_string(), "test".to_string()), true);
    }

    #[test]
    fn test_no_match() {
        assert_eq!(Solution::is_match("abc".to_string(), "def".to_string()), false);
        assert_eq!(Solution::is_match("a".to_string(), "b".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("a".to_string(), "aa".to_string()), false);
    }

    #[test]
    fn test_dot_single_char() {
        assert_eq!(Solution::is_match("a".to_string(), ".".to_string()), true);
        assert_eq!(Solution::is_match("b".to_string(), ".".to_string()), true);
        assert_eq!(Solution::is_match("z".to_string(), ".".to_string()), true);
    }

    #[test]
    fn test_dot_multiple_chars() {
        assert_eq!(Solution::is_match("abc".to_string(), "a.c".to_string()), true);
        assert_eq!(Solution::is_match("axc".to_string(), "a.c".to_string()), true);
        assert_eq!(Solution::is_match("abc".to_string(), "...".to_string()), true);
        assert_eq!(Solution::is_match("abc".to_string(), "a.b".to_string()), false);
    }

    #[test]
    fn test_star_zero_occurrences() {
        assert_eq!(Solution::is_match("b".to_string(), "a*b".to_string()), true);
        assert_eq!(Solution::is_match("ac".to_string(), "a*ac".to_string()), true);
        assert_eq!(Solution::is_match("b".to_string(), "b*b".to_string()), true);
    }

    #[test]
    fn test_star_one_occurrence() {
        assert_eq!(Solution::is_match("ab".to_string(), "a*b".to_string()), true);
        assert_eq!(Solution::is_match("aab".to_string(), "a*ab".to_string()), true);
    }

    #[test]
    fn test_star_multiple_occurrences() {
        assert_eq!(Solution::is_match("aab".to_string(), "a*b".to_string()), true);
        assert_eq!(Solution::is_match("aaab".to_string(), "a*b".to_string()), true);
        assert_eq!(Solution::is_match("aaaaab".to_string(), "a*b".to_string()), true);
    }

    #[test]
    fn test_dot_star() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
        assert_eq!(Solution::is_match("abc".to_string(), ".*c".to_string()), true);
        assert_eq!(Solution::is_match("abc".to_string(), "a.*c".to_string()), true);
        assert_eq!(Solution::is_match("abcdef".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn test_complex_patterns() {
        assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
        assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()), true);
        assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*c".to_string()), false);
    }

    #[test]
    fn test_multiple_stars() {
        assert_eq!(Solution::is_match("aaa".to_string(), "a*a".to_string()), true);
        assert_eq!(Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), "a*".to_string()), false); // empty string not allowed per constraints
    }

    #[test]
    fn test_pattern_longer_than_string() {
        assert_eq!(Solution::is_match("a".to_string(), "a*a*a*a*b".to_string()), false);
        assert_eq!(Solution::is_match("ab".to_string(), ".*..c*".to_string()), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
        assert_eq!(Solution::is_match("bbbba".to_string(), ".*a*a".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), ".*..".to_string()), false);
        assert_eq!(Solution::is_match("ab".to_string(), ".*..".to_string()), true);
    }

    #[test]
    fn test_single_char_strings() {
        assert_eq!(Solution::is_match("a".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("b".to_string(), "a*b".to_string()), true);
        assert_eq!(Solution::is_match("c".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn test_all_same_char() {
        assert_eq!(Solution::is_match("aaaa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("aaaa".to_string(), "a*.".to_string()), true);
        assert_eq!(Solution::is_match("aaaa".to_string(), ".a*".to_string()), true);
    }

    #[test]
    fn test_alternating_patterns() {
        assert_eq!(Solution::is_match("abab".to_string(), "a*b*a*b*".to_string()), true);
        assert_eq!(Solution::is_match("ababab".to_string(), "a*b*a*b*a*b*".to_string()), true);
        assert_eq!(Solution::is_match("abcd".to_string(), "a.*d".to_string()), true);
    }
}
