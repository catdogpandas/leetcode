/*
 * @lc app=leetcode.cn id=399 lang=rust
 *
 * [399] 除法求值
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut parent = (0..equations.len() * 2).collect::<Vec<_>>();
        let mut weight = vec![1.0_f64; equations.len() * 2];
        fn find(parent: &mut Vec<usize>, weight: &mut Vec<f64>, idx: usize) -> usize {
            if idx != parent[idx] {
                let p = find(parent, weight, parent[idx]);
                weight[idx] *= weight[parent[idx]];
                parent[idx] = p;
            }
            parent[idx]
        }
        fn union(
            parent: &mut Vec<usize>,
            weight: &mut Vec<f64>,
            idx1: usize,
            idx2: usize,
            val: f64,
        ) {
            let x = find(parent, weight, idx1);
            let y = find(parent, weight, idx2);
            if x == y {
                return;
            }
            parent[x] = y;
            weight[x] = val * weight[idx2] / weight[idx1];
        }
        let mut cnt = 0;
        let mut map = HashMap::new();
        equations
            .iter()
            .zip(values.iter())
            .for_each(|(item, &val)| {
                if map.get(&item[0]).is_none() {
                    map.insert(&item[0], cnt);
                    cnt += 1;
                }
                if map.get(&item[1]).is_none() {
                    map.insert(&item[1], cnt);
                    cnt += 1;
                }
                union(&mut parent, &mut weight, map[&item[0]], map[&item[1]], val);
            });
        queries
            .into_iter()
            .map(|item| {
                if map.get(&item[0]).is_none() || map.get(&item[1]).is_none() {
                    -1.0
                } else {
                    let idx1 = find(&mut parent, &mut weight, map[&item[0]]);
                    let idx2 = find(&mut parent, &mut weight, map[&item[1]]);
                    if idx1 == idx2 {
                        weight[map[&item[0]]] / weight[map[&item[1]]]
                    } else {
                        -1.0
                    }
                }
            })
            .collect()
    }
}
// @lc code=end
