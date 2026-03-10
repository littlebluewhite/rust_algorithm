pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let zero = zero as usize;
    let one = one as usize;
    let limit = limit as usize;
    let mut end0 = vec![vec![0; one + 1]; zero + 1];
    let mut end1 = vec![vec![0; one + 1]; zero + 1];
    for i in 1..=zero.min(limit) {
        end0[i][0] = 1;
    }
    for i in 1..=one.min(limit) {
        end1[0][i] = 1;
    }
    for z in 1..=zero {
        for o in 1..=one {
            let mut way0 = end1[z - 1][o] + end0[z - 1][o];
            // end0[z][o] = end1[z-1][o] + end1[z-2][o]+ ... + end1[z-limit][o];
            // end0[z-1][o] = end1[z-2][o] + end1[z-3][o] + ... + end1[z-limit-1][o];
            if z > limit {
                way0 -= end1[z - limit - 1][o];
            }
            way0 %= MOD;
            if way0 < 0 {
                way0 += MOD;
            }
            end0[z][o] = way0;

            let mut way1 = end0[z][o - 1] + end1[z][o - 1];
            if o > limit {
                way1 -= end0[z][o - limit - 1];
            }
            way1 %= MOD;
            if way1 < 0 {
                way1 += MOD;
            }
            end1[z][o] = way1;
        }
    }
    ((end0[zero][one] + end1[zero][one]) % MOD) as i32
}
