/*
 * @lc app=leetcode.cn id=805 lang=rust
 *
 * [805] 数组的均值分割
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let all_sum: i32 = nums.iter().sum();
        let n = nums.len();
        let m = n / 2;
        let mut ans = false;
        for i in 1..=m {
            if all_sum * i as i32 % n as i32 == 0 {
                ans = true;
                break;
            }
        }
        if !ans {
            return false;
        }
        //let mut dp = vec![HashSet::<i32>::new(); m + 1];
        let mut dp: Vec<HashSet<i32>> = vec![HashSet::<i32>::new(); m + 1];
        dp[0].insert(0);
        for num in nums {
            for i in (1..=m).rev() {
                let tmp = dp[i - 1].clone();
                for x in &tmp {
                    let curr = *x + num;
                    if curr * n as i32 == all_sum * i as i32 {
                        return true;
                    }
                    dp[i].insert(curr);
                }
            }
        }
        false
    }
}
// @lc code=end
