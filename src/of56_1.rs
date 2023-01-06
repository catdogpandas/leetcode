/*
 * @lc app=leetcode.cn id=剑指 Offer 56 - I lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 56 - I] 数组中数字出现的次数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |res, &x| res ^ x);
        let mut base = 1;
        for _ in 0..32 {
            if xor & base > 0 {
                break;
            }
            base <<= 1;
        }
        let res = nums.iter().fold((0, 0), |res, &x| {
            return if x & base > 0 {
                (res.0, res.1 ^ x)
            } else {
                (res.0 ^ x, res.1)
            };
        });

        vec![res.0, res.1]
    }
}
// @lc code=end

/*
// @lcpr case=start
// [4,1,4,6]\n
// @lcpr case=end

// @lcpr case=start
// [1,2,10,4,1,4,3,3]\n
// @lcpr case=end

 */
