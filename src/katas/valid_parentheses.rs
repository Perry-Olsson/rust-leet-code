pub struct Solution {}

impl Solution {
   pub fn is_valid(s: &str) -> bool {
        let mut char_stack: Vec<char> = Vec::new();
        for char in s.chars() {
            if char.is_opening() {
                char_stack.push(char)
            } else {
                match char_stack.last() {
                    Some(opening_char) if *opening_char == char.get_opening_char() => {
                        char_stack.pop();
                    },
                    _ => return false,
                }
            }
        }
        char_stack.is_empty()
    }
}

trait Bracket {
    fn is_opening(&self) -> bool;

    fn get_opening_char(&self) -> Self;
}

impl Bracket for char {
    fn is_opening(&self) -> bool {
        match self {
            '(' => true,
            '{' => true,
            '[' => true,
            _ => false
        }
    }

    fn get_opening_char(&self) -> char {
        match self {
            ')' => '(',
            '}' => '{',
            ']' => '[',
            _ => panic!("Shouldn't happen") 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_valid("(){({})}"));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_valid("(){({}})"));
    }
}
