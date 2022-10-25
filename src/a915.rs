/*
 * @lc app=leetcode.cn id=915 lang=rust
 *
 * [915] 分割数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut cur = i32::MAX;
        let nums_min: Vec<i32> = nums
            .iter()
            .rev()
            .map(|x| {
                cur = cur.min(*x);
                cur
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        cur = i32::MIN;
        for i in 0..nums.len() - 1 {
            cur = cur.max(nums[i]);
            if cur <= nums_min[i + 1] {
                return i as i32 + 1;
            }
        }
        0
    }
}
// @lc code=end
