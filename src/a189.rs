/*
 * @lc app=leetcode.cn id=189 lang=rust
 *
 * [189] 轮转数组
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for i in 0..nums.len() / 2 {
            let tmp = nums.len() - 1 - i;
            nums.swap(i, tmp)
        }
        let k = k % nums.len() as i32;
        for i in 0..k as usize / 2 {
            nums.swap(i, k as usize - 1 - i)
        }
        for i in 0..(nums.len() - k as usize) / 2 {
            let tmp = nums.len() - i - 1;
            nums.swap(i + k as usize, tmp);
        }
    }
}
// @lc code=end
