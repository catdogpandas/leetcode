/*
 * @lc app=leetcode.cn id=剑指 Offer 57 lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 57] 和为s的两个数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            if nums[i] > target / 2 {
                break;
            }
            let res = target - nums[i];
            if let Ok(res_id) = nums.binary_search(&res) {
                if res == nums[i]
                    && (nums.get(res_id) == nums.get(res_id + 1)
                        || nums.get(res_id) == nums.get(res_id - 1))
                {
                    return vec![res, res];
                } else {
                    return vec![nums[i], res];
                }
            }
        }
        vec![]
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,7,11,15]\n9\n
// @lcpr case=end

// @lcpr case=start
// [10,26,30,31,47,60]\n40\n
// @lcpr case=end

 */
