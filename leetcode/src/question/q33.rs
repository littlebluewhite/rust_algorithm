pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut left = 0i32;
    let mut right = n as i32 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        let l = left as usize;
        let r = right as usize;
        let m = mid as usize;
        if nums[m] == target {
            return mid;
        } else if nums[m] >= nums[l] {
            if target < nums[m] && target >= nums[l] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if target > nums[m] && target <= nums[r] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}
