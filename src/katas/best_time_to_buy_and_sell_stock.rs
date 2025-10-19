pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut lowest_price = prices[0];
        for &price in prices.iter().skip(1) {
            lowest_price = std::cmp::min(price, lowest_price);
            max_profit = std::cmp::max(price - lowest_price, max_profit)
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let max_profit = Solution::max_profit(prices);
        assert_eq!(5, max_profit);
    }

    #[test]
    fn test_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let max_profit = Solution::max_profit(prices);
        assert_eq!(0, max_profit);
    }
}
