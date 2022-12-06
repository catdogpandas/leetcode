/*
 * @lc app=leetcode.cn id=764 lang=rust
 *
 * [764] 最大加号标志
 */
struct Solution;
// @lc code=start
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
};
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let mut mine = HashSet::new();
        let mut tmp = HashMap::new();

        for item in mines {
            mine.insert((item[0], item[1]));
        }
        fn calc(i: i32, j: i32, mine: &HashSet<(i32, i32)>, n: i32) -> i32 {
            let mut ans = 0;
            while (i - ans >= 0) && (i + ans < n) && (j - ans >= 0) && (j + ans < n) {
                if mine.contains(&(i - ans, j))
                    || mine.contains(&(i, j + ans))
                    || mine.contains(&(i + ans, j))
                    || mine.contains(&(i, j - ans))
                {
                    return ans;
                }
                ans += 1;
            }
            ans
        }
        let mut cur_max = -1;
        for l in (1..=((n + 1) / 2)).rev() {
            if cur_max >= l {
                return cur_max;
            }
            for i in (l - 1)..=(n - l) {
                for j in (l - 1)..=(n - l) {
                    if let Some(res) = tmp.get(&(i, j)) {
                        if *res >= l {
                            return *res;
                        }
                    } else {
                        let res = calc(i, j, &mine, n);
                        if res >= l {
                            return res;
                        }
                        cur_max = cur_max.max(res);
                        tmp.insert((i, j), res);
                    }
                }
            }
        }
        0
    }
}
// @lc code=end
