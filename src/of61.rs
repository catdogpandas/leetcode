/*
 * @lc app=leetcode.cn id=面试题61 lang=rust
 * @lcpr version=21104
 *
 * [面试题61] 扑克牌中的顺子
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort();
        let mut zero_cnt = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                zero_cnt += 1;
            }
        }
        match zero_cnt {
            3 => {
                return if (nums[3] - nums[4]).abs() <= 4 && nums[4] != nums[3] {
                    true
                } else {
                    false
                }
            }
            2 => {
                return if nums[4] - nums[2] <= 4 && nums[4] != nums[3] && nums[2] != nums[3] {
                    true
                } else {
                    false
                }
            }
            1 => {
                return if nums[4] - nums[1] <= 4
                    && nums[4] != nums[3]
                    && nums[2] != nums[3]
                    && nums[2] != nums[1]
                {
                    true
                } else {
                    false
                }
            }
            0 => {
                return if nums[4] - nums[0] == 4
                    && nums[4] != nums[3]
                    && nums[2] != nums[3]
                    && nums[2] != nums[1]
                    && nums[0] != nums[1]
                {
                    true
                } else {
                    false
                }
            }
            _ => return true,
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
//
// @lcpr case=end

 */
