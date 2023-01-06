/*
 * @lc app=leetcode.cn id=剑指 Offer 59 - I lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 59 - I] 滑动窗口的最大值
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        if nums.is_empty() {
            return ans;
        }
        for i in 0..nums.len() {
            if i < k as usize {
                if queue.is_empty() {
                    queue.push_back(i);
                } else {
                    while !queue.is_empty() && nums[*queue.front().unwrap()] <= nums[i] {
                        queue.pop_front();
                    }
                    queue.push_front(i);
                }
                continue;
            }
            ans.push(nums[*queue.back().unwrap()]);
            while !queue.is_empty() && nums[*queue.front().unwrap()] <= nums[i] {
                queue.pop_front();
            }
            queue.push_front(i);
            while !queue.is_empty() && i - *queue.back().unwrap() >= k as usize {
                queue.pop_back();
            }
        }
        ans.push(nums[*queue.back().unwrap()]);

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,3,-1,-3,5,3,6,7]\n3\n
// @lcpr case=end

 */
