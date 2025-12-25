pub struct Solution {}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman_numeral = String::with_capacity(20);
        while num > 0 {
            match num {
                1000.. => {
                    roman_numeral.push_str("M");
                    num -= 1000;
                }
                900.. => {
                    roman_numeral.push_str("CM");
                    num -= 900;
                }
                500.. => {
                    roman_numeral.push_str("D");
                    num -= 500;
                }
                400.. => {
                    roman_numeral.push_str("CD");
                    num -= 400;
                }
                100.. => {
                    roman_numeral.push_str("C");
                    num -= 100;
                }
                90.. => {
                    roman_numeral.push_str("XC");
                    num -= 90;
                }
                50.. => {
                    roman_numeral.push_str("L");
                    num -= 50;
                }
                40.. => {
                    roman_numeral.push_str("XL");
                    num -= 40;
                }
                10.. => {
                    roman_numeral.push_str("X");
                    num -= 10;
                }
                9.. => {
                    roman_numeral.push_str("IX");
                    num -= 9;
                }
                5.. => {
                    roman_numeral.push_str("V");
                    num -= 5;
                }
                4.. => {
                    roman_numeral.push_str("IV");
                    num -= 4;
                }
                1.. => {
                    roman_numeral.push_str("I");
                    num -= 1;
                }
                _ => panic!("Shouldn't be possible")
            };
        }
        roman_numeral
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::int_to_roman(1), "I");
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
    }

    #[test]
    fn test_single_symbols() {
        assert_eq!(Solution::int_to_roman(1), "I");
        assert_eq!(Solution::int_to_roman(5), "V");
        assert_eq!(Solution::int_to_roman(10), "X");
        assert_eq!(Solution::int_to_roman(50), "L");
        assert_eq!(Solution::int_to_roman(100), "C");
        assert_eq!(Solution::int_to_roman(500), "D");
        assert_eq!(Solution::int_to_roman(1000), "M");
    }

    #[test]
    fn test_subtractive_notation() {
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(40), "XL");
        assert_eq!(Solution::int_to_roman(90), "XC");
        assert_eq!(Solution::int_to_roman(400), "CD");
        assert_eq!(Solution::int_to_roman(900), "CM");
    }

    #[test]
    fn test_repeating_symbols() {
        assert_eq!(Solution::int_to_roman(2), "II");
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(20), "XX");
        assert_eq!(Solution::int_to_roman(30), "XXX");
        assert_eq!(Solution::int_to_roman(200), "CC");
        assert_eq!(Solution::int_to_roman(300), "CCC");
        assert_eq!(Solution::int_to_roman(2000), "MM");
        assert_eq!(Solution::int_to_roman(3000), "MMM");
    }

    #[test]
    fn test_common_numbers() {
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
        assert_eq!(Solution::int_to_roman(27), "XXVII");
        assert_eq!(Solution::int_to_roman(621), "DCXXI");
        assert_eq!(Solution::int_to_roman(444), "CDXLIV");
    }

    #[test]
    fn test_numbers_with_multiple_subtractive_cases() {
        assert_eq!(Solution::int_to_roman(49), "XLIX");
        assert_eq!(Solution::int_to_roman(94), "XCIV");
        assert_eq!(Solution::int_to_roman(494), "CDXCIV");
        assert_eq!(Solution::int_to_roman(944), "CMXLIV");
        assert_eq!(Solution::int_to_roman(999), "CMXCIX");
    }

    #[test]
    fn test_round_numbers() {
        assert_eq!(Solution::int_to_roman(100), "C");
        assert_eq!(Solution::int_to_roman(500), "D");
        assert_eq!(Solution::int_to_roman(1000), "M");
        assert_eq!(Solution::int_to_roman(1500), "MD");
        assert_eq!(Solution::int_to_roman(2500), "MMD");
    }

    #[test]
    fn test_numbers_near_boundaries() {
        assert_eq!(Solution::int_to_roman(99), "XCIX");
        assert_eq!(Solution::int_to_roman(101), "CI");
        assert_eq!(Solution::int_to_roman(499), "CDXCIX");
        assert_eq!(Solution::int_to_roman(501), "DI");
        assert_eq!(Solution::int_to_roman(999), "CMXCIX");
        assert_eq!(Solution::int_to_roman(1001), "MI");
        assert_eq!(Solution::int_to_roman(3998), "MMMCMXCVIII");
    }
}

