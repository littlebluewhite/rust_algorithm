pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let n = nums.len();
    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i];
    }

    let mut min_sum = i32::MAX;
    for left in 0..n {
        for right in (left + l as usize)..=(left + r as usize) {
            if right > n {
                break;
            }
            let sum = prefix[right] - prefix[left];
            if sum > 0 {
                min_sum = min_sum.min(sum);
            }
        }
    }
    if min_sum == i32::MAX {
        -1
    } else {
        min_sum
    }
}
