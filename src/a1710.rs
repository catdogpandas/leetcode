/*
 * @lc app=leetcode.cn id=1710 lang=rust
 *
 * [1710] 卡车上的最大单元数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut ans = 0;
        let mut box_types = box_types;
        box_types.sort_by(|a, b| a[1].cmp(&b[1]).reverse());
        let mut cnt = 0;
        for item in box_types {
            if truck_size - cnt >= item[0] {
                ans += item[0] * item[1];
                cnt += item[0];
            } else if truck_size > cnt {
                ans += (truck_size - cnt) * item[1];
                break;
            } else {
                break;
            }
        }
        ans
    }
}
// @lc code=end
