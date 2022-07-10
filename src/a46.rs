/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */
struct Solution {}
// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>, visit: &mut Vec<i32>) {
            if tmp.len()==nums.len(){
                ans.push(tmp.to_vec());
                return;
            }
            for i in 0..nums.len(){
                if visit[i]==0{
                    visit[i]=1;
                    tmp.push(nums[i]);
                    dfs(nums, ans, tmp, visit);
                    tmp.pop();
                    visit[i]=0;
                }
            }
        }
        let mut ans = vec![];
        let mut tmp = vec![];
        let mut visit = vec![0; nums.len()];
        dfs(&nums, &mut ans, &mut tmp, &mut visit);
        ans
    }
}
// @lc code=end
