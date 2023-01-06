/*
 * @lc app=leetcode.cn id=剑指 Offer 56 - II lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 56 - II] 数组中数字出现的次数 II
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for item in nums {
            ones = ones ^ item & !twos;
            twos = twos ^ item & !ones;
        }
        ones
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,4,3,3]\n
// @lcpr case=end

// @lcpr case=start
// [9,1,7,9,7,9,7]\n
// @lcpr case=end

 */
