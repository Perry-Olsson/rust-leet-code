pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(num_rows as usize);
        result.push(vec![1]);
        for i in 1..num_rows {
            let row_size = (i + 1) as usize;
            let mut row: Vec<i32> = Vec::with_capacity(row_size);
            let prev_row = result[(i - 1) as usize].as_slice();
            for i in 0..row_size {
                let mut num = 0;
                if i > 0 {
                    num += prev_row[i - 1];
                }
                if i < prev_row.len() {
                    num += prev_row[i];
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
