pub fn distinct_subseq_ii(s: String) -> i32 {
    const MOD: i64 = 1000000007;
    let mut total = 0i64;
    let mut count = vec![0i64; 26];
    for ch in s.as_bytes() {
        let idx = (ch - b'a') as usize;
        let old = count[idx];
        let new = (total + 1) % MOD;
        total = (total + new - old + MOD) % MOD;
        count[idx] = new;
    }
    total as i32
}
