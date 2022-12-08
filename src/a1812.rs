/*
 * @lc app=leetcode.cn id=1812 lang=rust
 * @lcpr version=20801
 *
 * [1812] 判断国际象棋棋盘中一个格子的颜色
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let coo = coordinates.bytes().collect::<Vec<_>>();
        match (coo[0] as char, coo[1] % 2) {
            ('a' | 'c' | 'e' | 'g', 0) | ('b' | 'd' | 'f' | 'h', 1) => true,
            ('a' | 'c' | 'e' | 'g', 1) | ('b' | 'd' | 'f' | 'h', 0) => false,

            (_, _) => false,
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "a1"\n
// @lcpr case=end

// @lcpr case=start
// "h3"\n
// @lcpr case=end

// @lcpr case=start
// "c7"\n
// @lcpr case=end

 */
