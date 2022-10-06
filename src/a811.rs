/*
 * @lc app=leetcode.cn id=811 lang=rust
 *
 * [811] 子域名访问计数
 */
struct Solution {}
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut domains_map: HashMap<&str, i32> = HashMap::new();
        for item in cpdomains.iter() {
            if let Some((a, mut b)) = item.split_once(" ") {
                let count = a.parse::<i32>().unwrap();
                *domains_map.entry(b).or_insert(0) += count;
                while let Some((_, domain)) = b.split_once(".") {
                    *domains_map.entry(domain).or_insert(0) += count;
                    b = domain;
                }
            }
        }
        domains_map
            .iter()
            .map(|(&key, &val)| val.to_string() + " " + key)
            .collect()
    }
}
// @lc code=end
