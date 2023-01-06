/*
 * @lc app=leetcode.cn id=剑指 Offer 46 lang=rust
 * @lcpr version=21001
 *
 * [剑指 Offer 46] 把数字翻译成字符串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        let mut a = 1;
        let mut b = 0;
        let s_num = num.to_string().chars().collect::<Vec<_>>();
        for i in 0..s_num.len() {
            if i == 0 {
                b = 1;
            } else {
                match (s_num[i - 1], s_num[i]) {
                    ('1', _) => {
                        let tmp = a + b;
                        a = b;
                        b = tmp;
                    }
                    ('2', '0' | '1' | '2' | '3' | '4' | '5') => {
                        let tmp = a + b;
                        a = b;
                        b = tmp;
                    }
                    (_, _) => {
                        a = b;
                    }
                }
            }
        }
        b
    }
}
// @lc code=end

/*
// @lcpr case=start
//
// @lcpr case=end

 */
