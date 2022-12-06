/*
 * @lc app=leetcode.cn id=621 lang=rust
 *
 * [621] 任务调度器
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        for item in &tasks {
            *cnt.entry(*item).or_insert(0) += 1;
        }
        let max_exec = cnt.iter().fold(0, |max, item| max.max(*item.1));
        let max_count = cnt.iter().fold(
            0,
            |res, item| if *item.1 == max_exec { res + 1 } else { res },
        );
        (tasks.len() as i32).max((max_exec - 1) * (n + 1) + max_count)
    }
}
// @lc code=end
