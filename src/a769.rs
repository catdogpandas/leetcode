/*
 * @lc app=leetcode.cn id=769 lang=rust
 *
 * [769] 最多能完成排序的块
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max = i32::MIN;

        arr.iter().enumerate().fold(0, |ans, (idx, &item)| {
            max = max.max(item);
            if max == idx as i32 {
                return ans + 1;
            }
            ans
        })
    }
}
// @lc code=end
