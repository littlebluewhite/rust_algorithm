pub fn find_min(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left = 0;
    let mut right = n-1;
    while left < right {
        let mid = (left + right) / 2;
        if nums[right] < nums[mid]{
            left = mid + 1;
        }else{
            right = mid;
        }
    }
    nums[left]
}