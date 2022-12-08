/*
 * @lc app=leetcode.cn id=1687 lang=rust
 * @lcpr version=20603
 *
 * [1687] 从仓库到码头运输箱子
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn box_delivering(
        boxes: Vec<Vec<i32>>,
        ports_count: i32,
        max_boxes: i32,
        max_weight: i32,
    ) -> i32 {
        let mut tmp_count = 0;
        let mut tmp_weight = 0;
        let mut tmp_port = -1;
        let mut boxes_d = vec![];
        for i in 0..boxes.len() {
            if i + 1 < boxes.len() && boxes[i][0] == boxes[i + 1][0] {
                tmp_count += 1;
                tmp_weight += boxes[i][1];
                tmp_port = boxes[i][0];
            } else {
                if tmp_port > 0 {
                    boxes_d.push((tmp_port, tmp_count, tmp_weight));
                } else {
                    boxes_d.push((boxes[i][0], 1, boxes[i][1]));
                }
                tmp_port = -1;
                tmp_count = 0;
                tmp_weight = 0;
            }
        }

        0
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[1,1],[2,1],[1,1]]\n2\n3\n3\n
// @lcpr case=end

// @lcpr case=start
// [[1,2],[3,3],[3,1],[3,1],[2,4]]\n3\n3\n6\n
// @lcpr case=end

// @lcpr case=start
// [[1,4],[1,2],[2,1],[2,1],[3,2],[3,4]]\n3\n6\n7\n
// @lcpr case=end

// @lcpr case=start
// [[2,4],[2,5],[3,1],[3,2],[3,7],[3,1],[4,4],[1,3],[5,2]]\n5\n5\n7\n
// @lcpr case=end

 */
