pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut count = 0;
    let mut prefix = vec![0i32; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i];
        if nums[i] == target {
            prefix[i + 1] += 1;
            count += 1;
        }
    }
    if count == 0 {
        return 0;
    }
    let mut ans = 0;
    for l in 0..n {
        let mut r = l;
        while r < n {
            let subarray_count = prefix[r + 1] - prefix[l];
            if (r-l+1)as i32/2 < subarray_count{
                ans += 1;
            }
            r += 1;
        }
    }
    ans
}