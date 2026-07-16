pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let size = *nums.iter().max().unwrap_or(&0) as usize;
    let mut dp = vec![vec![0; size + 1]; size + 1];
    dp[0][0] = 1;
    for x in nums {
        let mut next = dp.clone();
        let x = x as usize;
        for i in 0..=size {
            for j in 0..=size {
                let way = dp[i][j];
                if way == 0 {
                    continue;
                }
                let g1 = gcd(i, x);
                next[g1][j] = (way + next[g1][j]) % MOD;

                let g2 = gcd(j, x);
                next[i][g2] = (way + next[i][g2]) % MOD;
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for i in 1..=size {
        ans = (ans + dp[i][i]) % MOD;
    }
    ans as i32
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
