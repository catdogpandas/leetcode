/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut cnt = HashMap::new();
        nums.iter().for_each(|x| {
            *cnt.entry(*x).or_insert(0) += 1;
        });
        let mut cnt = cnt.into_iter().collect::<Vec<_>>();
        fn quick_select(cnt: &mut Vec<(i32, i32)>, l: usize, r: usize, target: usize) {
            if l >= cnt.len() || r >= cnt.len() {
                return;
            }
            let p_val = cnt[r].1;
            let mut p = l;
            for i in l..r {
                if cnt[i].1 <= p_val {
                    cnt.swap(i, p);
                    p += 1;
                }
            }
            cnt.swap(p, r);
            if p == target {
                return;
            } else if p < target {
                quick_select(cnt, p + 1, r, target);
            } else {
                quick_select(cnt, l, p - 1, target);
            }
        }
        let len = cnt.len();
        quick_select(&mut cnt, 0, len - 1, len - k as usize);
        cnt.split_at(cnt.len() - k as usize)
            .1
            .iter()
            .map(|item| item.0)
            .collect()
    }
}
// @lc code=end
