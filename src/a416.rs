/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().sum::<i32>() as usize;
        let max = *nums.iter().max().unwrap() as usize;
        if sum % 2 != 0 {
            return false;
        }
        sum = sum / 2;
        if max > sum {
            return false;
        }
        let mut dp = vec![false; sum + 1];
        dp[0] = true;
        for i in 0..nums.len() {
            let num = nums[i] as usize;
            for j in (num..=sum).rev() {
                dp[j] |= dp[j - num];
            }
        }
        dp[sum]
    }
}
// @lc code=end
