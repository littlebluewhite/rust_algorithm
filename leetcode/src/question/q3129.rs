pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let zero = zero as usize;
    let one = one as usize;
    let limit = limit as usize;
    let mut dp_0 = vec![vec![0; one + 1]; zero + 1];
    let mut dp_1 = vec![vec![0; one + 1]; zero + 1];
    for i in 0..=zero.min(limit) {
        dp_0[i][0] = 1;
    }
    for i in 0..=one.min(limit) {
        dp_1[0][i] = 1;
    }
    for z in 1..=zero {
        for o in 1..=one {
            let start_z = z.saturating_sub(limit);
            let mut way_0 = 0i64;
            for i in start_z..z {
                way_0 += dp_1[i][o];
                if way_0 > MOD {
                    way_0 -= MOD;
                }
                dp_0[z][o] = way_0;
            }

            let start_o = o.saturating_sub(limit);
            let mut way_1 = 0i64;
            for i in start_o..o {
                way_1 += dp_0[z][i];
                if way_1 > MOD {
                    way_1 -= MOD;
                }
                dp_1[z][o] = way_1
            }
        }
    }
    ((dp_0[zero][one] + dp_1[zero][one]) % MOD) as i32
}
