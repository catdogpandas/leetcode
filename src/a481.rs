/*
 * @lc app=leetcode.cn id=481 lang=rust
 *
 * [481] 神奇字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut base = vec![1];
        let (mut slow, mut fast) = (0, 0);
        while (slow as i32) < n && (fast as i32) < n {
            let count = base.get(slow + 1);
            let last = base.last();
            match (count, last) {
                (None, Some(last)) => {
                    if *last == 1 {
                        base.push(2);
                        base.push(2);
                    } else {
                        base.push(1);
                    }
                }
                (Some(count), Some(last)) => {
                    if *last == 1 {
                        for _ in 0..*count as usize {
                            base.push(2);
                        }
                    } else {
                        for _ in 0..*count as usize {
                            base.push(1);
                        }
                    }
                }
                (_, _) => {}
            }
            slow += 1;
        }
        let mut ans = 0;
        for i in 0..n as usize {
            if base[i] == 1 {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
