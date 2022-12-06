/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let k = k as usize;
        let mut queue = VecDeque::new();
        let mut ans = vec![];

        for i in 0..nums.len() {
            while !queue.is_empty() {
                let last = *queue.back().unwrap();
                if nums[last] >= nums[i] {
                    queue.push_back(i);
                    break;
                }
                queue.pop_back();
            }
            if queue.is_empty() {
                queue.push_back(i);
            }
            if i + 1 < k {
                continue;
            }
            while !queue.is_empty() {
                let last = *queue.front().unwrap();
                if i - last + 1 > k {
                    queue.pop_front();
                } else {
                    ans.push(nums[last]);
                    break;
                }
            }
        }

        ans
    }
}
// @lc code=end
