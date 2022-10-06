/*
 * @lc app=leetcode.cn id=870 lang=rust
 *
 * [870] 优势洗牌
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn advantage_count(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        let mut idx2 = vec![0; nums2.len()];
        for i in 0..nums2.len() {
            idx2[i] = i;
        }
        idx2.sort_by(|&a, &b| nums2[a].cmp(&nums2[b]));
        //nums2.sort();
        let mut l = 0;
        let mut r = nums2.len() - 1;
        let mut ans = vec![0; nums2.len()];
        for i in 0..nums2.len() {
            if nums1[i] <= nums2[idx2[l]] {
                ans[idx2[r]] = nums1[i];
                r -= 1;
            } else {
                ans[idx2[l]] = nums1[i];
                l += 1;
            }
        }
        ans
    }
}
// @lc code=end
