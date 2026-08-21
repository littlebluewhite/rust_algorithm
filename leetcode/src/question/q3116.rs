pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
    let coins: Vec<i64> = coins.into_iter().map(|x| x as i64).collect();
    let terms = build_terms(&coins);
    let k = k as i64;
    let min = *coins.iter().min().unwrap();
    let mut lo = min;
    let mut hi = min * k;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if count_amount(&terms, mid) >= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn build_terms(coins: &[i64]) -> Vec<(i64, i64)> {
    let n = coins.len();
    let mut terms = Vec::with_capacity((1 << n) - 1);
    for mask in 1usize..1 << n {
        let mut lcm_n = 1;
        let mut bits = 0;
        for j in 0..n {
            if mask & (1 << j) != 0 {
                bits += 1;
                lcm_n = lcm(coins[j], lcm_n);
            }
        }
        let sign = if bits % 2 == 1 { 1 } else { -1 };
        terms.push((lcm_n, sign));
    }
    terms
}

fn count_amount(terms: &[(i64, i64)], target: i64) -> i64 {
    let mut total = 0;
    for &(lcm_n, sign) in terms {
        total += sign * (target / lcm_n);
    }
    total
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
