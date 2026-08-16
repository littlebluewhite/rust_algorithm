pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut has_non_zero = false;
    let mut total = 0;
    for x in nums{
        total ^= x;
        if x != 0{
            has_non_zero = true;
        }
    }
    if total != 0{
        return n as i32
    }else if has_non_zero{
        return (n - 1) as i32
    }else{
        return 0
    }
}