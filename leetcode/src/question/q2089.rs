pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    let mut ans = Vec::with_capacity(n);
    nums.sort_unstable();
    for i in 0..n{
        if nums[i] == target{
            ans.push(i as i32);
        }else if nums[i] > target{
            break
        }
    }
    ans
}