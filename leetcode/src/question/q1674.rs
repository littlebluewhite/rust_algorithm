pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
    let n = nums.len();
    let limit = limit as usize;
    let mut diff = vec![0; 2 * limit + 2];
    for i in 0..n / 2 {
        let a = nums[i] as usize;
        let b = nums[n - 1 - i] as usize;
        let low = a.min(b) + 1;
        let high = a.max(b) + limit;
        let sum = (a + b);
        diff[2] += 2;
        diff[limit * 2 + 1] -= 2;
        diff[low] -= 1;
        diff[high + 1] += 1;
        diff[sum] -= 1;
        diff[sum + 1] += 1;
    }

    let mut moved = 0;
    let mut ans = i32::MAX;
    for i in 2..limit * 2 + 1 {
        moved += diff[i];
        ans = ans.min(moved);
    }
    ans
}
