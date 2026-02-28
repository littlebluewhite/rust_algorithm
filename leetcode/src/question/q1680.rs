pub fn concatenated_binary(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 0i64;
    let mut bit_move = 0;
    for i in 1..=n{
        let i = i as i64;
        if (i & (i-1)) == 0{
            bit_move += 1;
        }
        ans = ((ans << bit_move) | i) % MOD;
    }
    ans as i32
}