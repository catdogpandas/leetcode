/*
 * @lc app=leetcode.cn id=剑指 Offer 57 - II lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 57 - II] 和为s的连续正数序列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let table = (0..target / 2 + 3).into_iter().collect::<Vec<_>>();
        let mut cur_sum = 0;
        let mut l = 1;
        let mut r = 1;
        while l <= r && r <= (target / 2 + 2) {
            if cur_sum < target {
                cur_sum += r;
                r += 1;
            } else if cur_sum == target {
                ans.push((table[l as usize..r as usize]).to_vec());
                cur_sum -= l;
                l += 1;
            } else {
                cur_sum -= l;
                l += 1;
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 9\n
// @lcpr case=end

// @lcpr case=start
// 15\n
// @lcpr case=end

 */
