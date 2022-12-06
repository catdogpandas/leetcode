/*
 * @lc app=leetcode.cn id=剑指 Offer 11 lang=rust
 * @lcpr version=20702
 *
 * [剑指 Offer 11] 旋转数组的最小数字
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        if numbers[0] < *numbers.last().unwrap() {
            return numbers[0];
        }
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l <= r && r < numbers.len() {
            let mid = (l + r) / 2;

            if numbers[mid] > numbers[r] {
                l = mid + 1;
            } else if numbers[mid] < numbers[r] {
                r = mid;
            } else {
                r -= 1;
            }
        }
        numbers[l]
    }
}
// @lc code=end

/*
// @lcpr case=start
// [3,4,5,1,2]\n
// @lcpr case=end

// @lcpr case=start
// [2,2,2,0,1]\n
// @lcpr case=end

 */
