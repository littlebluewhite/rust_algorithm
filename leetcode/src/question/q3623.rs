use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;
const INV2: i64 = (MOD + 1) / 2;

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let mut freq = HashMap::new();
    for point in points{
        let y = point[1];
        *freq.entry(y).or_insert(0) += 1;
    }
    let mut sum_b = 0i64;
    let mut sum_b2 = 0i64;
    for &v in freq.values(){
        if v < 2{
            continue;
        }
        let total_line = (v*(v-1)/2) % MOD;
        sum_b += total_line;
        sum_b2 += ((total_line as i128 * total_line as i128) % MOD as i128) as i64;
    }
    let mut ans = ((sum_b as i128 * sum_b as i128) % MOD as i128) as i64;
    ans -= sum_b2 % MOD;
    if ans < 0{
        ans += MOD;
    }
    ans = (ans * INV2) % MOD;
    ans as i32
}