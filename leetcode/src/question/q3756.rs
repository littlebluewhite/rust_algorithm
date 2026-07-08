pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    const MOD: i64 = 1000000007;
    let mut res: Vec<i32> = Vec::with_capacity(queries.len());
    let b: Vec<i64> = s.as_bytes().iter().map(|&c| (c - b'0')as i64).collect();
    let n = b.len();
    let mut pow10 = vec![1; n + 1];
    for i in 0..n {
        pow10[i+1] = pow10[i] * 10 % MOD;
    }
    let mut non_zero_count: Vec<usize> = vec![0; n+ 1];

    let mut prefix_sum: Vec<i64> = vec![0; n + 1];
    let mut value_mul: Vec<i64> = Vec::with_capacity(n + 1);
    value_mul.push(0);
    for i in 0..n{
        prefix_sum[i+1] = prefix_sum[i] + b[i];
        non_zero_count[i+1] = non_zero_count[i];
        if b[i] != 0 {
            non_zero_count[i+1] += 1;
            let next_x = (value_mul[value_mul.len()-1] * 10 % MOD + b[i])%MOD;
            value_mul.push(next_x);
        }
    }
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        let left = non_zero_count[l];
        let right = non_zero_count[r+1];
        let len = right - left;
        let sum = prefix_sum[r+1] - prefix_sum[l];
        let mut x = value_mul[right] - value_mul[left]*pow10[len]%MOD;
        if x < 0 {
            x += MOD;
        }
        res.push(((x * sum) % MOD) as i32);
    }
    res
}
