/*
 * @lc app=leetcode.cn id=733 lang=rust
 *
 * [733] 图像渲染
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        fn dfs(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32, scolor: i32) {
            if sr < 0 || sr >= image.len() as i32 || sc < 0 || sc >= image[0].len() as i32 {
                return;
            }
            if image[sr as usize][sc as usize] == scolor {
                image[sr as usize][sc as usize] = color;
                dfs(image, sr + 1, sc, color, scolor);
                dfs(image, sr - 1, sc, color, scolor);
                dfs(image, sr, sc + 1, color, scolor);
                dfs(image, sr, sc - 1, color, scolor);
            }
        }
        let mut image = image;
        if color == image[sr as usize][sc as usize] {
            return image;
        }
        let scolor = image[sr as usize][sc as usize];
        dfs(&mut image, sr, sc, color, scolor);
        image
    }
}
// @lc code=end
