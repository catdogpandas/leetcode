/*
 * @lc app=leetcode.cn id=862 lang=rust
 *
 * [862] 和至少为 K 的最短子数组
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let (mut ret, mut pre_sum, mut queue) = (i64::MAX, 0, VecDeque::new());
        queue.push_back((0, -1));
        for i in 0..nums.len() {
            pre_sum += nums[i] as i64;
            while !queue.is_empty() && pre_sum <= queue[queue.len() - 1].0 {
                queue.pop_back();
            }
            while !queue.is_empty() && pre_sum - queue[0].0 >= k as i64 {
                ret = ret.min(i as i64 - queue.pop_front().unwrap().1 as i64);
            }
            queue.push_back((pre_sum, i as i32));
        }
        if ret == i64::MAX {
            -1
        } else {
            ret as i32
        }
    }
}
// @lc code=end
