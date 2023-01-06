/*
 * @lc app=leetcode.cn id=1827 lang=rust
 * @lcpr version=20801
 *
 * [1827] 最少操作使数组递增
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut base = 0;
        let mut ans = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                base = nums[i - 1] - nums[i] + 1;
                nums[i] = nums[i - 1] + 1;
            }
            ans += base;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,1,1]\n
// @lcpr case=end

// @lcpr case=start
// [1,5,2,4,1]\n
// @lcpr case=end

// @lcpr case=start
// [8]\n
// @lcpr case=end

 */
