pub fn is_good(mut nums: Vec<i32>) -> bool {
    let n = nums.len();
    nums.sort_unstable();

    for i in 0..n{
        if i == n-1{
            if nums[n-1] != n as i32 - 1 {
                return false;
            }
        }else{
            if nums[i] != i as i32+1 {
                return false;
            }
        }
    }
    true
}