pub fn smallest_palindrome(s: String, k: i32) -> String {
    let mut freq = vec![0; 26];
    let b = s.as_bytes();
    for &ch in b {
        freq[(ch - b'a') as usize] += 1;
    }
    let mut middle = None;
    let mut half_count = vec![0; 26];
    for i in 0..26{
        half_count[i] = freq[i]/2;
        if freq[i] % 2 == 1 {
            middle = Some(i);
        }
    }
    let half_len = s.len() / 2;
    let (primes, factors) = build_factorizations(half_len);
    let mut exps = vec![0i32; primes.len()];
    for value in 2..=half_len{
        add_factorization(&mut exps, &factors[value], 1);
    }
    for &count in &half_count{
        for value in 2..=count{
            add_factorization(&mut exps, &factors[value], -1);
        }
    }
    let mut rank = k as u64;
    if capped_value(&exps, &primes, rank) < rank{
        return String::new();
    }

    let mut remaining = half_len;
    let mut left = Vec::with_capacity(half_len);
    while remaining > 0{
        let mut pick = false;
        for i in 0..26{
            if half_count[i] == 0{
                continue;
            }
            apply_pick_delta(&mut exps, &factors, half_count[i], remaining, 1);
            let block = capped_value(&exps, &primes, rank);
            apply_pick_delta(&mut exps, &factors, half_count[i], remaining, -1);
            if block < rank{
                rank -= block;
                continue;
            }
            apply_pick_delta(&mut exps, &factors, half_count[i], remaining, 1);
            half_count[i] -= 1;
            remaining -= 1;
            pick = true;
            left.push(b'a' + i as u8);
            break;
        }
        if !pick{
            return String::new();
        }
    }
    let mut result = Vec::with_capacity(s.len());
    result.extend_from_slice(&left);
    if let Some(i) = middle{
        result.push(b'a' + i as u8);
    }
    for &b in left.iter().rev(){
        result.push(b);
    }
    String::from_utf8(result).unwrap()
}

fn build_factorizations(mut n: usize)-> (Vec<u64>, Vec<Vec<(usize, i32)>>){
    let mut primes_values = vec![];
    let mut prime_index = vec![usize::MAX; n + 1];
    let mut factors = vec![vec![]; n + 1];
    for value in 2..=n{
        let mut is_prime = true;
        let mut d = 2;
        while d *d <= value {
            if value % d == 0{
                is_prime = false;
                break;
            }
            d += 1;
        }
        if is_prime{
            prime_index[value] = primes_values.len();
            primes_values.push(value);
        }
    }
    for value in 2..=n{
        let mut rest = value;
        for (idx, prime) in primes_values.iter().enumerate(){
            if prime * prime > value{
                break;
            }
            let mut count = 0;
            while rest % prime == 0{
                rest /= prime;
                count += 1;
            }
            if count > 0{
                factors[value].push((idx, count));
            }
        }
        if rest > 1 {
            factors[value].push((prime_index[rest], 1));
        }
    }
    let primes: Vec<u64> = primes_values.into_iter().map(|prime| prime as u64).collect();
    (primes, factors)
}

fn add_factorization(exps: &mut Vec<i32>, factors: &[(usize, i32)], sign: i32){
    for &(idx, count) in factors{
        exps[idx] += sign*count;
    }
}

fn capped_value(exps: &Vec<i32>, primes: &Vec<u64>, mut rank: u64)-> u64{
    let over = rank + 1;
    let mut value = 1;
    for i in 0..exps.len(){
        let prime = primes[i];
        let count = exps[i];
        for _ in 0..count{
            if value * prime >rank{
                return over
            }
            value *= prime;
        }
    }
    value
}

fn apply_pick_delta(exps: &mut Vec<i32>, factors: &[Vec<(usize, i32)>], count: usize, remaining: usize, sign: i32){
    add_factorization(exps, &factors[count], sign);
    add_factorization(exps, &factors[remaining], -sign);
}

