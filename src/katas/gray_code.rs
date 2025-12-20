pub struct Solution {}

impl Solution {
   pub fn gray_code(s: i32) -> Vec<i32> {
       let mut result: Vec<i32> = Vec::with_capacity(1 << s);
       result.push(0);
       result.push(1);
       for n in 1..s {
           let power = (2 as i32).pow(n as u32);
           for num in (0..result.len()).rev() {
               result.push(result[num] + power)
           }
       }
       result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::gray_code(1))
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2))
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 1, 3, 2, 6, 7, 5, 4], Solution::gray_code(3))
    }
}
/*
00 
01
11
10

*/
