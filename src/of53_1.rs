/*
 * @lc app=leetcode.cn id=剑指 Offer 53 - I lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 53 - I] 在排序数组中查找数字 I
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let idx = nums.binary_search(&target);
        let mut ans = 0;
        if let Ok(x) = idx {
            for i in x..nums.len() {
                if nums[i] == target {
                    ans += 1;
                } else {
                    break;
                }
            }
            for i in (0..x).rev() {
                if nums[i] == target {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [5,7,7,8,8,10]\n8\n
// @lcpr case=end

// @lcpr case=start
// [5,7,7,8,8,10]\n6\n
// @lcpr case=end

 */
