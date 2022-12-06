/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // 拓扑排序
        use std::collections::{HashMap, VecDeque};
        let mut in_degree = HashMap::new();
        let mut out_degree = HashMap::new();
        for item in &prerequisites {
            let (a, b) = (item[0], item[1]);
            *in_degree.entry(a).or_insert(0) += 1;
            in_degree.entry(b).or_insert(0);
            out_degree.entry(b).or_insert(vec![]).push(a);
        }
        let mut queue = VecDeque::new();
        in_degree.iter().for_each(|item| {
            if *item.1 == 0 {
                queue.push_back(*item.0);
            }
        });
        while !queue.is_empty() {
            let course = queue.pop_front().unwrap();
            out_degree.entry(course).or_default().iter().for_each(|c| {
                *in_degree.entry(*c).or_default() -= 1;
                if *in_degree.entry(*c).or_default() == 0 {
                    queue.push_back(*c);
                }
            });
        }
        in_degree.into_iter().all(|j| j.1 == 0)
    }
}
// @lc code=end
