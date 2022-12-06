/*
 * @lc app=leetcode.cn id=791 lang=rust
 *
 * [791] 自定义字符串排序
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        s.sort_by(|a, b| match (order.find(*a), order.find(*b)) {
            (Some(ia), Some(ib)) => ia.cmp(&ib),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (_, _) => a.cmp(b),
        });
        s.iter().collect()
    }
}
// @lc code=end
