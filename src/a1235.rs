/*
 * @lc app=leetcode.cn id=1235 lang=rust
 *
 * [1235] 规划兼职工作
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let (mut jobs, mut max_profit) = (
            (0..start_time.len())
                .map(|i| vec![start_time[i], end_time[i], profit[i], profit[i]])
                .collect::<Vec<_>>(),
            0,
        );
        jobs.sort_by(|a, b| a[1].cmp(&b[1]));

        fn binary_search(jobs: &Vec<Vec<i32>>, index: usize) -> Option<usize> {
            let (mut l, mut r) = (0, index as i32 - 1);
            while l <= r {
                let m = l + ((r - l) >> 1) as i32;
                if jobs[m as usize][1] <= jobs[index][0] {
                    if jobs[m as usize + 1][1] <= jobs[index][0] {
                        l = m + 1;
                    } else {
                        return Some(m as usize);
                    }
                } else {
                    r = m - 1;
                }
            }
            None
        }

        for i in 1..start_time.len() {
            jobs[i][3] = jobs[i][2].max(jobs[i - 1][3]);
            if let Some(l) = binary_search(&jobs, i) {
                jobs[i][3] = jobs[i][3].max(jobs[i][2] + jobs[l][3]);
            }
            max_profit = max_profit.max(jobs[i][3]);
        }
        max_profit
    }
}
// @lc code=end
