use std::mem::swap;

/*
 * @lc app=leetcode.cn id=剑指 Offer 21 lang=rust
 * @lcpr version=21006
 *
 * [剑指 Offer 21] 调整数组顺序使奇数位于偶数前面
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut l = 0;
        for i in 0..nums.len() {
            if nums[i] % 2 == 1 {
                nums.swap(i, l);
                l += 1;
            }
        }
        nums
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,4]\n
// @lcpr case=end

 */
