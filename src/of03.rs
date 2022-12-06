/*
 * @lc app=leetcode.cn id=剑指 Offer 03 lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 03] 数组中重复的数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut ptr = 0;
        while ptr < nums.len() as i32 {
            if nums[ptr as usize] == ptr {
                ptr += 1;
                continue;
            }
            if nums[nums[ptr as usize] as usize] == nums[ptr as usize] {
                return nums[ptr as usize];
            }
            let tmp = nums[ptr as usize];
            nums.swap(ptr as usize, tmp as usize);
        }

        0
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

 */
