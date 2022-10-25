/*
 * @lc app=leetcode.cn id=901 lang=rust
 *
 * [901] 股票价格跨度
 */

// @lc code=start
struct StockSpanner {
    stock: Vec<i32>,
    stack: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self {
            stock: vec![],
            stack: vec![],
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.stock.push(price);
        let mut res = 1;
        while let Some(idx) = self.stack.last() {
            res = self.stock.len() - 1 - *idx;
            if self.stock[*idx] > price {
                self.stack.push(self.stock.len() - 1);
                break;
            } else {
                self.stack.pop();
            }
        }
        if self.stack.is_empty() {
            res = self.stock.len();
            self.stack.push(self.stock.len() - 1);
        }
        res as i32
    }
}

// @lc code=end
