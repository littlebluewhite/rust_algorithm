pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut left = 0i32;
    let mut right = 0i32;

    for i in 0..nums.len(){
        right += nums[i];
    }

    for i in 0..nums.len(){
        right -= nums[i];
        if nums[i] == 0{
            if left-right == 0 {
                ans += 2;
            }else if (left-right).abs() <2 {
                ans += 1;
            }
        }
        left += nums[i];
    }
    ans
}