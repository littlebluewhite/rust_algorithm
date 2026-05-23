pub fn check(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut pre = nums[n-1];
    let mut count = 0;
    for i in 0..n{
        if nums[i] < pre {
            count += 1;
            if count > 1 {
                return false;
            }
        }
        pre = nums[i];
    }
    true
}