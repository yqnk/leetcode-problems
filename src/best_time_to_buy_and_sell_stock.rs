struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut buy = prices[0];

        for i in 1..prices.len() {
            profit = std::cmp::max(profit, prices[i] - buy);
            buy = std::cmp::min(buy, prices[i]);
        }
        profit
    }
}

fn main() {
    let prices = vec![2, 1, 2, 0, 1];
    let result = Solution::max_profit(prices);
    println!("The result is {}", result);
}
