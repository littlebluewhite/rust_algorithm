pub fn missing_integer(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    for i in 1..nums.len(){
        if nums[i] == nums[i-1]+1{
            sum += nums[i];
        }else{
            break;
        }
    }
    while nums.contains(&sum){
        sum += 1;
    }
    sum
}