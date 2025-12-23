pub struct Solution {}


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_numeral_values: [i32; 89] = [0; 89];
        roman_numeral_values[73] = 1;
        roman_numeral_values[86] = 5;
        roman_numeral_values[88] = 10;
        roman_numeral_values[76] = 50;
        roman_numeral_values[67] = 100;
        roman_numeral_values[68] = 500;
        roman_numeral_values[77] = 1000;


        let mut result = 0;
        let mut i = 0;
        let bytes = s.as_bytes();
        while i < s.len() - 1 {
            let val = roman_numeral_values[usize::from(bytes[i])];
            let next_val = roman_numeral_values[usize::from(bytes[i + 1])];
            if next_val > val {
                result -= val;
            } else {
                result += val;
            }
            i += 1;
        }
        result += roman_numeral_values[usize::from(bytes[i])];
        result
    }
}

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::roman_to_int("I".to_string()));
    }

    #[test]
    fn test_5() {
        assert_eq!(2994, Solution::roman_to_int("MMCMXCIV".to_string()));
    }

    #[test]
    fn test_6() {
        assert_eq!(2995, Solution::roman_to_int("MMCMXCV".to_string()));
    }

    #[test]
    fn test_7() {
        assert_eq!(2999, Solution::roman_to_int("MMCMXCIX".to_string()));
    }

    #[test]
    fn test_8() {
        assert_eq!(40004, Solution::roman_to_int("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMIV".to_string()));
    }
}
