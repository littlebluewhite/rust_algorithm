pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let n = nums.len();
    let mut left = 0usize;
    let mut right = n-1;
    let mut ans = vec![pivot; n];
    for i in 0..n{
        if nums[i] < pivot{
            ans[left] = nums[i];
            left += 1;
        }
        let j = n-1-i;
        if nums[j] > pivot{
            ans[right] = nums[j];
            right -= 1;
        }
    }
    ans
}