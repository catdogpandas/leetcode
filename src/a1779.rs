/*
 * @lc app=leetcode.cn id=1779 lang=rust
 *
 * [1779] 找到最近的有相同 X 或 Y 坐标的点
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .enumerate()
            .fold((-1, i32::MAX), |res, (idx, item)| {
                if item[0] == x {
                    let d = (item[1] - y).abs();
                    if d < res.1 {
                        return (idx as i32, d);
                    }
                }
                if item[1] == y {
                    let d = (item[0] - x).abs();
                    if d < res.1 {
                        return (idx as i32, d);
                    }
                }
                res
            })
            .0
    }
}
// @lc code=end
