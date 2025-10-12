pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];
        for i in 1..num_rows {
            let size = (i + 1) as usize;
            let mut row: Vec<i32> = Vec::with_capacity(size);
            let prev = &result[(i - 1) as usize];
            for i in 0..size {
                let mut num = 0;
                let idx1 = i as i32 - 1;
                let idx2 = i;
                if idx1 >= 0 {
                    num += prev[idx1 as usize];
                }
                if idx2 < prev.len() {
                    num += prev[idx2];
                }
                row.push(num);
            }
            result.push(row);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let pascals_triange = Solution::generate(1);
        assert_eq!(pascals_triange, vec![vec![1]]);
    }

    #[test]
    fn test_2() {
        let pascals_triange = Solution::generate(2);
        assert_eq!(pascals_triange, vec![vec![1], vec![1, 1]]);
    }
    #[test]
    fn test_3() {
        let pascals_triange = Solution::generate(5);
        assert_eq!(pascals_triange, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
    }
}
