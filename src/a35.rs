/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(val) => return val as i32,
            Err(val) => return val as i32,
        }
    }
}
// @lc code=end
