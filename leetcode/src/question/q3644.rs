pub fn sort_permutation(nums: Vec<i32>) -> i32 {
    let mut common = -1i32;
    for (i, value) in nums.into_iter().enumerate(){
        if value != i as i32 {
            common &= value;
        }
    }
    common.max(0)
}