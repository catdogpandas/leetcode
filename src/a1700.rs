/*
 * @lc app=leetcode.cn id=1700 lang=rust
 *
 * [1700] 无法吃午餐的学生数量
 */
struct Solution;
// @lc code=start

use std::collections::VecDeque;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students.iter().collect::<VecDeque<_>>();
        let mut iter = sandwiches.iter();
        while let Some(val) = iter.next() {
            let mut cnt = students.len();
            while cnt > 0 {
                let catage = students.pop_front().unwrap();
                if catage == val {
                    break;
                } else {
                    students.push_back(catage);
                }
                cnt -= 1;
            }
            if cnt == 0 {
                break;
            }
        }
        students.len() as i32
    }
}
// @lc code=end
