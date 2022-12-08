/*
 * @lc app=leetcode.cn id=167 lang=rust
 *
 * [167] 两数之和 II - 输入有序数组
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        //双指针
        //因为是有序数组 两个指针左右向中间靠拢即可
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            if numbers[left] + numbers[right] > target {
                right -= 1;
            } else if (numbers[left] + numbers[right] == target) {
                break;
            } else {
                left += 1;
            }
        }

        vec![left as i32 + 1, right as i32 + 1]
    }
}
// @lc code=end
