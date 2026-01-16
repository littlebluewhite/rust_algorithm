use std::collections::{HashSet};
const MOD: i64 = 1000000007;

pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
    let mut h: Vec<i64> = h_fences.into_iter().map(|x| x as i64).collect();
    let mut v: Vec<i64> = v_fences.into_iter().map(|x| x as i64).collect();
    h.push(1);
    h.push(m as i64);
    v.push(1);
    v.push(n as i64);
    h.sort_unstable();
    v.sort_unstable();

    let mut hset: HashSet<i64> = HashSet::new();
    for i in 0..h.len() {
        for j in (i + 1)..h.len() {
            hset.insert(h[j] - h[i]);
        }
    }

    let mut max_side: i64 = -1;
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            let d = v[j] - v[i];
            if hset.contains(&d) && d > max_side {
                max_side = d;
            }
        }
    }

    if max_side < 0 {
        return -1;
    }
    let area = (max_side % MOD) * (max_side % MOD) % MOD;
    area as i32
}
