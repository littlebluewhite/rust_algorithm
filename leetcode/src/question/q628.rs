pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut max1 = i32::MIN;
    let mut max2 = i32::MIN;
    let mut max3 = i32::MIN;
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;
    for i in 0..nums.len() {
        if nums[i] >= max1 {
            max3 = max2;
            max2 = max1;
            max1 = nums[i];
        }else if nums[i] >= max2 {
            max3 = max2;
            max2 = nums[i];
        }else if nums[i] >= max3 {
            max3 = nums[i];
        }
        if nums[i] <= min1 {
            min2 = min1;
            min1 = nums[i];
        }else if nums[i] <= min2 {
            min2 = nums[i];
        }
    }
    (max1 * max2 * max3).max(min1 * min2 * max1)
}