use std::ops::Deref;

/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut target = 0;
        for item in nums.iter() {
            if item == &target {
                count += 1;
            } else if count == 0 {
                count = 1;
                target = *item;
            } else {
                count -= 1;
            }
        }

        target
    }
}
// @lc code=end
