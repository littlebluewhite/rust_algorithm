fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn min_count(mut t: i64) -> usize {
    let mut k = 9i64;
    let mut count = 0usize;
    while t > 1 {
        if t % k == 0 {
            t /= k;
            count += 1;
        } else {
            k -= 1;
        }
    }
    count
}

fn build_suffix(mut need: i64, len: usize) -> Vec<u8> {
    let mut res = vec![b'1'; len];
    let mut k = 9i64;
    for i in (0..len).rev() {
        if need == 1 {
            break;
        }
        while need % k != 0 {
            k -= 1;
        }
        need /= k;
        res[i] = b'0' + k as u8;
    }
    res
}

pub fn smallest_number(num: String, t: i64) -> String {
    let mut residual = t;
    for p in [2i64, 3, 5, 7] {
        while residual % p == 0 {
            residual /= p;
        }
    }
    if residual != 1 {
        return "-1".to_string();
    }
    let bytes = num.as_bytes();
    let n = bytes.len();
    let mut rem = vec![0i64; n + 1];
    rem[0] = t;
    let mut first_zero = n;
    for i in 0..n {
        if bytes[i] == b'0' {
            first_zero = i;
            break;
        }
        rem[i + 1] = rem[i] / gcd((bytes[i] - b'0') as i64, rem[i]);
    }

    if first_zero == n && rem[n] == 1 {
        return num;
    }

    let last_pivot = if first_zero == n { n - 1 } else { first_zero };
    for i in (0..=last_pivot).rev() {
        let suffix_len = n - i - 1;
        for digit in ((bytes[i] - b'0') as i64 + 1)..=9 {
            let need = rem[i] / gcd(digit, rem[i]);
            if min_count(need) <= suffix_len {
                let mut ans = Vec::with_capacity(n);
                ans.extend_from_slice(&bytes[..i]);
                ans.push(digit as u8 + b'0');
                ans.extend_from_slice(&build_suffix(need, suffix_len));
                return String::from_utf8(ans).unwrap();
            }
        }
    }
    let len = (n + 1).max(min_count(t));
    String::from_utf8(build_suffix(t, len)).unwrap()
}
