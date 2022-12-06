/*
 * @lc app=leetcode.cn id=907 lang=rust
 *
 * [907] 子数组的最小值之和
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut left = vec![0; arr.len() + 1];
        let mut right = vec![0; arr.len() + 1];

        for i in 0..arr.len() {
            left[i] = i;
            let n = arr[i];
            while stack.len() > 0 && arr[stack[stack.len() - 1]] >= n {
                let idx = stack.pop().unwrap();
                left[i] = left[idx];
                right[idx] = i;
            }
            stack.push(i);
        }

        for idx in stack.iter() {
            right[*idx as usize] = arr.len();
        }

        let mut ans: i64 = 0;
        for i in 0..arr.len() {
            let n = arr[i] as i64;
            ans += (i - left[i] + 1) as i64 * (right[i] - i) as i64 * n
        }

        return (ans % 1000000007) as i32;
    }
}
// @lc code=end
