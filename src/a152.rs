/*
 * @lc app=leetcode.cn id=152 lang=rust
 *
 * [152] 乘积最大子数组
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut dp0 = -1;
        let mut dp1 = 1;
        for i in 0..nums.len() {
            if i == 0 {
                ans = ans.max(nums[i]);
                if nums[i] > 0 {
                    dp0 = nums[i];
                } else if nums[i] < 0 {
                    dp1 = nums[i];
                }
            } else {
                match (dp0, dp1, nums[i]) {
                    (-1, b, c) => {
                        if c > 0 {
                            dp0 = c;
                            dp1 = b * c;
                        } else {
                            dp0 = b * c;
                            dp1 = c;
                        }
                    }
                    (a, 1, c) => {
                        if c > 0 {
                            dp0 = c.max(a * c);
                            dp1 = 1;
                        } else {
                            dp0 = -1;
                            dp1 = c.min(a * c);
                        }
                    }
                    (a, b, c) if c >= 0 => {
                        dp0 = c.max(c * a);
                        dp1 = c.min(c * b);
                    }
                    (a, b, c) if c < 0 => {
                        dp0 = c.max(c * b);
                        dp1 = c.min(c * a);
                    }
                    (_, _, _) => {}
                }
                if dp0 != -1 {
                    ans = ans.max(dp0);
                }
            }
        }
        ans
    }
}
// @lc code=end
