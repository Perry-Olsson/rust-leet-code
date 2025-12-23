pub struct Solution {}

impl Solution {
   pub fn is_valid(s: String) -> bool {
        let mut char_stack: Vec<u8> = Vec::with_capacity(s.len());
        for char in s.bytes() {
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

impl Bracket for u8 {
    fn is_opening(&self) -> bool {
        match self {
            b'(' => true,
            b'{' => true,
            b'[' => true,
            _ => false
        }
    }

    fn get_opening_char(&self) -> u8 {
        match self {
            b')' => b'(',
            b'}' => b'{',
            b']' => b'[',
            _ => panic!("Shouldn't happen") 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_valid("(){({})}".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_valid("(){({}})".to_string()));
    }
}
