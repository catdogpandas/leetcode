/*
 * @lc app=leetcode.cn id=剑指 Offer 48 lang=rust
 * @lcpr version=21001
 *
 * [剑指 Offer 48] 最长不含重复字符的子字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        let s = s.into_bytes();
        let mut freq = vec![0; 256];
        let mut answer = 0;
        let mut lo = 0;
        for hi in 0..n {
            freq[s[hi] as usize] += 1;
            if freq[s[hi] as usize] > 1 {
                while s[lo] != s[hi] {
                    freq[s[lo] as usize] -= 1;
                    lo += 1;
                }
                freq[s[lo] as usize] -= 1;
                lo += 1;
            }
            answer = answer.max(hi - lo + 1);
        }
        answer as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
//
// @lcpr case=end

// @lcpr case=start
//
// @lcpr case=end

 */
