pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    let mut total = 0i32;
    for &n in nums.iter() {
        total += is_divisors_return_sum(n);
    }
    total
}

fn is_divisors_return_sum(n: i32) -> i32 {
    let mut count = 0i32;
    let mut total = 0i32;
    let mut i = 1i32;
    while i * i <= n {
        if n % i == 0 {
            let j = n / i;
            if i == j {
                count += 1;
                total += i;
            } else {
                count += 2;
                total += i + j;
            }
        }
        if count > 4 {
            return 0;
        }
        i += 1;
    }
    if count == 4 { total } else { 0 }
}
