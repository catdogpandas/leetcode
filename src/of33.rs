/*
 * @lc app=leetcode.cn id=剑指 Offer 33 lang=rust
 * @lcpr version=21105
 *
 * [剑指 Offer 33] 二叉搜索树的后序遍历序列
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        fn verify_postorder(postorder: &[i32]) -> bool {
            if postorder.len() <= 2 {
                return true;
            }
            let head = *postorder.last().unwrap();
            let mut p = postorder.len() - 1;
            for i in 0..postorder.len() - 1 {
                if postorder[i] > head {
                    p = i;
                    break;
                }
            }
            for i in p..postorder.len() - 1 {
                if postorder[i] < head {
                    return false;
                }
            }
            verify_postorder(&postorder[0..p])
                && verify_postorder(&postorder[p..postorder.len() - 1])
        }
        verify_postorder(&postorder[..])
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

 */
