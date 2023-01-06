/*
 * @lc app=leetcode.cn id=剑指 Offer 40 lang=rust
 * @lcpr version=21104
 *
 * [剑指 Offer 40] 最小的k个数
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        fn select(arr: &mut Vec<i32>, l: usize, r: usize, k: usize) {
            if l >= arr.len() || r >= arr.len() {
                return;
            }
            let t = arr[r];
            let mut cnt = l;
            for i in l..r {
                if arr[i] <= t {
                    arr.swap(i, cnt);
                    cnt += 1;
                }
            }
            arr.swap(cnt, r);
            if cnt == k {
                return;
            } else if cnt > k {
                select(arr, l, cnt - 1, k);
            } else {
                select(arr, cnt + 1, r, k);
            }
        }
        let r = arr.len() - 1;
        select(&mut arr, 0, r, k as usize);
        arr[0..k as usize].to_vec()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,2,1]\n2\n
// @lcpr case=end

// @lcpr case=start
// [0,1,2,1]\n1\n
// @lcpr case=end

 */
