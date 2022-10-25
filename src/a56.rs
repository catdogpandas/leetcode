/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 * [56] 合并区间
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        intervals.sort();
        let mut left = -1;
        let mut right = -1;
        for item in intervals {
            if left < 0 {
                left = item[0];
                right = item[1];
            } else {
                if left == item[0] {
                    right = right.max(item[1]);
                } else {
                    if right >= item[0] {
                        right = right.max(item[1]);
                    } else {
                        ans.push(vec![left, right]);
                        left = item[0];
                        right = item[1];
                    }
                }
            }
        }
        ans.push(vec![left, right]);
        ans
    }
}
// @lc code=end
