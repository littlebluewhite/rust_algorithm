const MOD: i64 = 1_000_000_007;
pub fn count_permutations(complexity: Vec<i32>) -> i32 {
    let mut n = complexity.len();
    let min = complexity[0];
    for i in 1..n {
        if complexity[i] <= min {
            return 0;
        }
    }
    let mut ans = 1i64;
    n -= 1;
    while n > 0 {
        ans = (ans * n as i64) % MOD;
        n -= 1;
    }
    ans as i32
}
