const MOD: i64 = 1_000_000_007;
const MAX_VAL: usize = 100_000*2;

pub fn special_triplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right = vec![0i32; MAX_VAL+1];
    let mut left = vec![0i32; MAX_VAL+1];
    for &num in &nums{
        right[num as usize] += 1;
    }
    let mut ans = 0i64;
    for num in nums{
        right[num as usize] -= 1;
        let target = num * 2;
        ans = (ans + left[target as usize] as i64 * right[target as usize] as i64) % MOD;
        left[num as usize] += 1;
    }
    (ans % MOD) as i32
}