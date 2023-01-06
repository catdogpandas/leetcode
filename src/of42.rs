/*
 * @lc app=leetcode.cn id=剑指 Offer 42 lang=rust
 * @lcpr version=20801
 *
 * [剑指 Offer 42] 连续子数组的最大和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((i32::MIN, i32::MIN), |res, &x| {
                if res.0 == i32::MIN {
                    return (x, x);
                } else {
                    let l = x.max(res.0 + x);
                    return (l, l.max(res.1));
                }
            })
            .1
    }
}
// @lc code=end

/*
// @lcpr case=start
// [-2,1,-3,4,-1,2,1,-5,4]\n
// @lcpr case=end

 */
