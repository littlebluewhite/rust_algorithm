pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
    let mut ans = i64::MIN;
    let n = nums.len();
    let mut i = 0usize;

    while i < n {
        let mut j = i + 1;
        while j < n && nums[j] > nums[j - 1] {
            j += 1;
        }
        let p = j - 1;
        if p == i {
            i += 1;
            continue;
        }

        let mut sum = nums[p] as i64 + nums[p - 1] as i64;
        while j < n && nums[j] < nums[j - 1] {
            sum += nums[j] as i64;
            j += 1;
        }
        let q = j - 1;
        if q == p || j == n || (j < n && nums[j] == nums[q]) {
            i = q;
            continue;
        }
        sum += nums[q + 1] as i64;

        let mut max_sum = 0i64;
        let mut fix_sum = 0i64;
        let mut r = q + 2;
        while r < n && nums[r] > nums[r-1]{
            fix_sum += nums[r] as i64;
            max_sum = max_sum.max(fix_sum);
            r += 1;
        }
        sum += max_sum;

        max_sum = 0i64;
        fix_sum = 0i64;
        let mut l = p as isize - 2;
        while l >= i as isize{
            fix_sum += nums[l as usize] as i64;
            max_sum = max_sum.max(fix_sum);
            l -= 1
        }
        sum += max_sum;
        ans = ans.max(sum);
        i = q;
    }
    ans
}
