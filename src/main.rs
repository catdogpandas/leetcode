use std::collections::HashMap;
use std::collections::HashSet;
fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
    let mut adj:HashMap<i32,HashSet<i32>> = HashMap::new();
    for item in pairs {
        adj.entry(item[0]).or_insert(HashSet::new()).insert(item[1]);
        adj.entry(item[1]).or_insert(HashSet::new()).insert(item[0]);
    }
    //check root node
    let mut root = -1;
    let mut tmp = adj.len();
    for (node,neighbours) in &adj {
        if neighbours.len() == tmp -1 {
            root = *node;
            break;
        }
    }
    if root == -1 {
        return 0;
    }
    let mut res = 1;
    for (node,neighbours) in &adj {
        if *node == root {
            continue;
        }
        let currDegree = neighbours.len();
        let mut parent = -1;
        let mut parentDegree = usize::MAX;
        for neighbor in neighbours {
            if adj[neighbor].len() < parentDegree && adj[neighbor].len() >= currDegree {
                parent = *neighbor;
                parentDegree = adj[neighbor].len();
            }
        }
        if parent == -1 {
            return 0;
        }
        for neighbour in neighbours {
            if *neighbour == parent {
                continue;
            }
            if !adj.get(&parent).unwrap().contains(&neighbour) {
                return 0;
            }
        }
        if parentDegree== currDegree {
            res =2;
        }
    }
    return res;
}
/*
class Solution {
    public:
        int checkWays(vector<vector<int>>& pairs) {
            unordered_map<int, unordered_set<int>> adj;
            for (auto &p : pairs) {
                adj[p[0]].emplace(p[1]);
                adj[p[1]].emplace(p[0]);
            }
            /* 检测是否存在根节点*/
            int root = -1;
            for (auto &[node, neighbours] : adj) {
                if (neighbours.size() == adj.size() - 1) {
                    root = node;
                    break;
                }
            }
            if (root == -1) {
                return 0;
            }
    
            int res = 1;
            for (auto &[node, neighbours] : adj) {
                if (node == root) {
                    continue;
                }
                int currDegree = neighbours.size();
                int parent = -1;
                int parentDegree = INT_MAX;
    
                /* 根据 degree 的大小找到 node 的父节点 parent */
                for (auto &neighbour : neighbours) {
                    if (adj[neighbour].size() < parentDegree && adj[neighbour].size() >= currDegree) {
                        parent = neighbour;
                        parentDegree = adj[neighbour].size();
                    }
                }
                if (parent == -1) {
                    return 0;
                }
    
                /* 检测 neighbours 是否是 adj[parent] 的子集 */
                for (auto &neighbour : neighbours) {
                    if (neighbour == parent) {
                        continue;
                    }
                    if (!adj[parent].count(neighbour)) {
                        return 0;
                    }
                }
                if (parentDegree == currDegree) {
                    res = 2;
                }
            }
            return res;
        }
    };
    
    
    作者：LeetCode-Solution
    链接：https://leetcode-cn.com/problems/number-of-ways-to-reconstruct-a-tree/solution/zhong-gou-yi-ke-shu-de-fang-an-shu-by-le-36e1/
    来源：力扣（LeetCode）
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/
fn main() {
    let test = vec![vec![0, 3], vec![4, 8], vec![3, 5]];
}
