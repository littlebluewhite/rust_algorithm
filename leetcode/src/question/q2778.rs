pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let n = nums.len();
    for i in 0..n {
        if n % (i + 1) == 0 {
            sum += nums[i] * nums[i];
        }
    }
    sum
}