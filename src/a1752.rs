use core::num;

/*
 * @lc app=leetcode.cn id=1752 lang=rust
 *
 * [1752] 检查数组是否经排序和轮转得到
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut state = false;
        let cur_min = nums[0];
        let mut pre = nums[0];
        nums.iter().fold(true, |mut res, &item| {
            match state {
                false => {
                    if pre > item {
                        state = true;
                        if item > cur_min {
                            res = false;
                        }
                    }
                }
                true => {
                    if pre > item || item > cur_min {
                        res = false;
                    }
                }
            }
            pre = item;
            res
        })
    }
}
// @lc code=end
