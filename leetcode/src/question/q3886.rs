pub fn sortable_integers(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut drop = vec![0; n];
    let mut prefix_max = vec![i32::MIN; n + 1];
    let mut suffix_min = vec![i32::MAX; n + 1];
    for i in 0..n {
        prefix_max[i + 1] = prefix_max[i].max(nums[i]);
        if i > 0 {
            drop[i] = drop[i - 1] + (nums[i - 1] > nums[i]) as usize;
        }
    }
    for i in (0..n).rev() {
        suffix_min[i] = suffix_min[i + 1].min(nums[i]);
    }
    let is_sortable = |k: usize| -> bool {
        for l in (0..n).step_by(k) {
            let r = l + k - 1;
            let drop_num = drop[r] - drop[l] + (nums[l] < nums[r]) as usize;
            if drop_num > 1 {
                return false;
            }
            if r + 1 < n && prefix_max[r + 1] > suffix_min[r + 1] {
                return false;
            }
        }
        true
    };
    let mut ans = 0;
    let mut x = 1;
    while x <= n / x {
        if n % x == 0 && is_sortable(x) {
            ans += x as i32;
        }
        if n % x == 0 && x != n / x && is_sortable(n / x) {
            ans += (n / x) as i32;
        }
        x += 1;
    }
    ans
}
