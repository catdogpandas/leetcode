/*
 * @lc app=leetcode.cn id=1769 lang=rust
 *
 * [1769] 移动所有球到每个盒子所需的最小操作数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes
            .chars()
            .map(|c| match c {
                '0' => 0,
                _ => 1,
            })
            .collect::<Vec<_>>();
        let mut sum = 0;
        let mut cnt = 0;
        let prefix_cnt = boxes
            .iter()
            .map(|&x| {
                let res = cnt;
                cnt += x;
                res
            })
            .collect::<Vec<_>>();
        let prefix_sum = boxes
            .iter()
            .enumerate()
            .map(|x| {
                let res = sum;
                if *x.1 == 1 {
                    sum += x.0 as i32;
                }
                res
            })
            .collect::<Vec<_>>();
        boxes
            .iter()
            .enumerate()
            .map(|(idx, x)| sum - 2 * prefix_sum[idx] + (2 * prefix_cnt[idx] - cnt) * idx as i32)
            .collect()
    }
}
// @lc code=end
