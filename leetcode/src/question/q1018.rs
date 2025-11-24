pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut ans = Vec::new();
    let mut sum = 0i32;
    for num in nums {
        sum = ((sum << 1) + num) % 5;
        if sum == 0 {
            ans.push(true);
        } else {
            ans.push(false);
        }
    }
    ans
}
