/*
 * @lc app=leetcode.cn id=1773 lang=rust
 *
 * [1773] 统计匹配检索规则的物品数量
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let rule_key = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 0,
        };
        items.iter().fold(0, |sum, item| {
            if item[rule_key] == rule_value {
                return sum + 1;
            }
            sum
        })
    }
}
// @lc code=end
