pub struct Solution {}

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        Solution::_simple(column_number)
    }

    fn mine(column_number: i32) -> String {
        let mut result = String::new();
        let mut col_idx: i32 = column_number;

        while col_idx > 0 {
            let digit = col_idx % 26;
            if digit == 0 {
                result.push('Z');
                col_idx -= 1;
            } else {
                result.push(char::from((digit + 64) as u8));
            }
            if col_idx == 26 {
                break;
            }
            col_idx /= 26;
        }
        result.chars().rev().collect()
    }

    fn _simple(mut col: i32) -> String {
        let mut result = String::new();

        while col > 0 {
            col -= 1;
            let char = char::from((col % 26) as u8 + 'A' as u8);
            result.push(char);
            col /= 26;
        }
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("A".to_string(), Solution::convert_to_title(1));
    }

    #[test]
    fn test_2() {
        assert_eq!("AB".to_string(), Solution::convert_to_title(28));
    }

    #[test]
    fn test_3() {
        assert_eq!("BB".to_string(), Solution::convert_to_title(54));
    }

    #[test]
    fn test_4() {
        assert_eq!("ZY".to_string(), Solution::convert_to_title(701));
    }

    #[test]
    fn test_5() {
        assert_eq!("AZ".to_string(), Solution::convert_to_title(52));
    }

    #[test]
    fn test_6() {
        assert_eq!("ABZ".to_string(), Solution::convert_to_title(754));
    }
}
