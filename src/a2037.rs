/*
 * @lc app=leetcode.cn id=2037 lang=rust
 * @lcpr version=21104
 *
 * [2037] 使每位学生都有座位的最少移动次数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats = seats;
        seats.sort();
        let mut students = students;
        students.sort();
        seats
            .iter()
            .zip(students.iter())
            .fold(0, |res, (seat, student)| res + (seat - student).abs())
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,1,5]\n[2,7,4]\n
// @lcpr case=end

// @lcpr case=start
// [4,1,5,9]\n[1,3,2,6]\n
// @lcpr case=end

// @lcpr case=start
// [2,2,6,6]\n[1,3,2,6]\n
// @lcpr case=end

 */
