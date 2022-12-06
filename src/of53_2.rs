/*
 * @lc app=leetcode.cn id=剑指 Offer 53 - II lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 53 - II] 0～n-1中缺失的数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r && r < nums.len() {
            let mid = (l + r) / 2;
            if nums[mid] as usize == mid {
                l = mid + 1;
            } else if nums[mid] as usize > mid
                && (mid > 0 && nums[mid - 1] as usize == mid - 1 || mid == 0)
            {
                return mid as i32;
            } else {
                r = mid - 1;
            }
        }
        nums.len() as i32
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
