/*
 * @lc app=leetcode.cn id=剑指 Offer 39 lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 39] 数组中出现次数超过一半的数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |res, &x| {
                return if res.1 == 0 {
                    (x, 1)
                } else if res.0 == x {
                    (x, res.1 + 1)
                } else {
                    (res.0, res.1 - 1)
                };
            })
            .0
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

 */
