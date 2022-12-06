/*
 * @lc app=leetcode.cn id=1620 lang=rust
 *
 * [1620] 网络信号最好的坐标
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let quality = |x: i32, y: i32| {
            let mut q = 0;
            for item in &towers {
                let d = (x - item[0]).pow(2) + (y - item[1]).pow(2);
                if d <= radius.pow(2) {
                    q += (item[2] as f32 / (1f32 + (d as f32).sqrt())) as i32;
                }
            }
            q
        };
        let mut ans = (0, 0);
        let mut max = i32::MIN;
        for i in 0..=50 {
            for j in 0..=50 {
                let tmp = quality(i, j);
                if max < tmp {
                    max = tmp;
                    ans = (i, j);
                }
            }
        }

        vec![ans.0, ans.1]
    }
}
// @lc code=end
