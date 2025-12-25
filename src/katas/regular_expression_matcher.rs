pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut matchers: Vec<Matcher> = Vec::new();
        let mut p_iter = p.chars().peekable();
        while let Some(c) = p_iter.next() {
            let mut matcher = Matcher {
                chr: c,
                is_zero_or_more: false
            };
            if let Some(next) = p_iter.peek() {
                if *next == '*' {
                    matcher.is_zero_or_more = true;
                    p_iter.next();
                }
            }
            matchers.push(matcher);
        }
        println!("{:?}", matchers);
        false
    }
}

#[derive(Debug)]
struct Matcher {
    chr: char,
    is_zero_or_more: bool
}

trait CharMatcher {
    fn match_chr(&self, char: char) -> bool;
}

impl CharMatcher for char {
    fn match_chr(&self, char: char) -> bool {
        if *self == '.' {
            true
        } else {
            *self == char
        }
    }
}

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
    /* pub fn is_match(s: String, p: String) -> bool {
        let mut pattern = p.chars().peekable();
        let mut input = s.chars().peekable();
        while let Some(c) = pattern.next() {
            let next = pattern.peek();
            match next {
                Some(nc) if *nc == '*' => {
                    if *nc == '*' {
                        pattern.next();
                        if c == '.' {
                            // .* case
                            match pattern.peek() {
                                Some(next_pattern) => {
                                    while let Some(input_char) = input.peek() {
                                        if next_pattern.match_chr(*input_char) {
                                            break;
                                        } else {
                                            input.next();
                                        }
                                    }
                                }
                                None => {
                                    return true;
                                }
                            }
                        } else {
                            // char* case
                            println!("hello");
                            while let Some(input_char) = input.peek() {
                                if c.match_chr(*input_char) {
                                    input.next();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {
                    if let Some(input_char) = input.next() {
                        if !c.match_chr(input_char) {
                            return false;
                        }
                    } else {
                        //end of string return true or false?
                        return false;
                    }
                }
            }
        }
        input.next().is_none()
    } */
