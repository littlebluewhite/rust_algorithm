pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1000000007;
    let mut count = vec![0; 32];
    for mut num in nums {
        while num != 0 {
            let bit = num.trailing_zeros() as usize;
            count[bit] += 1;
            num &= num - 1
        }
    }
    let mut ans = 0i64;
    for _ in 0..k {
        let mut x = 0;
        for (bit, cnt) in count.iter_mut().enumerate() {
            if *cnt > 0 {
                x |= 1 << bit;
                *cnt -= 1;
            }
        }
        ans = (ans + x * x) % MOD;
    }
    ans as i32
}
