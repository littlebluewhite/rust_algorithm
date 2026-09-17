pub fn number_of_sets(n: i32, k: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let total = (n + k - 1) as usize;
    let choose = k as usize * 2;
    let mut comb = vec![0; choose + 1];
    comb[0] = 1;
    for i in 1..=total {
        let upper = choose.min(i);
        for j in (1..=upper).rev() {
            comb[j] = (comb[j] + comb[j - 1]) % MOD;
        }
    }
    comb[choose] as i32
}
