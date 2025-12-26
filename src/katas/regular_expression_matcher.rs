pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Solution::is_match_memo(
            0,
            0,
            &s.chars().collect::<Vec<_>>(),
            &p.chars().collect::<Vec<_>>(),
            &mut Memo::new(s.len(), p.len())
            )
    }

    fn is_match_memo(i: usize, j: usize, input: &[char], pattern: &[char], memo: &mut Memo) -> bool {
        println!("{:?}", memo);
        if let Some(result) = memo.get(i, j) {
            return result
        }

        let ans;
        if j == pattern.len() {
            ans = i == input.len();
        } else {
            let does_char_match = i < input.len() && pattern[j].match_chr(input[i]);

            if (j + 1) < pattern.len() && pattern[j + 1] == '*' {
                ans = Solution::is_match_memo(i, j + 2, input, pattern, memo) || 
                    (does_char_match && Solution::is_match_memo(i + 1, j, input, pattern, memo));
            } else {
                ans = does_char_match && Solution::is_match_memo(i + 1, j + 1, input, pattern, memo);
            }
        }

        memo.set(i, j, ans);
        ans
    }

    fn is_match_recursive(s: &[char], p: &[char]) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        let does_first_char_match = !s.is_empty() && p[0].match_chr(s[0]);
        if p.len() >= 2 && p[1] == '*' {
            return Solution::is_match_recursive(s, &p[2..]) || 
                (does_first_char_match && Solution::is_match_recursive(&s[1..], p))
        } else {
            return does_first_char_match && Solution::is_match_recursive(&s[1..], &p[1..]);
        }
    }
}

#[derive(Debug)]
struct Memo {
    memo: Vec<Vec<Option<bool>>>
}

impl Memo {
    fn new(width: usize, height: usize) -> Self {
        let mut vec: Vec<Vec<Option<bool>>> = Vec::with_capacity(width + 1);
        for _ in 0..(width + 1) {
            let mut inner: Vec<Option<bool>> = Vec::with_capacity(height + 1);
            for _ in 0..(height + 1) {
                inner.push(None);
            }
            vec.push(inner);
        }
        Memo { memo: vec }
    }

    fn get(&self, row: usize, col: usize) -> Option<bool> {
        let row = &self.memo[row];
        row[col]
    }

    fn set(&mut self, row: usize, col: usize, value: bool) {
        let row = &mut self.memo[row];
        row[col] = Some(value);
    }
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

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test] fn test_empty_args() {
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "".to_string()), false);
    }

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
        /* assert_eq!(Solution::is_match("aaab".to_string(), "a*b".to_string()), true);
        assert_eq!(Solution::is_match("aaaaab".to_string(), "a*b".to_string()), true); */
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
        assert_eq!(Solution::is_match("".to_string(), "a*".to_string()), true); // empty string not allowed per constraints
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
