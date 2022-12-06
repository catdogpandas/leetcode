/*
 * @lc app=leetcode.cn id=406 lang=rust
 *
 * [406] 根据身高重建队列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0]).reverse()
            }
        });
        let mut ans = vec![];
        for item in people {
            ans.insert(item[1] as usize, item);
        }

        ans
    }
}
// @lc code=end
