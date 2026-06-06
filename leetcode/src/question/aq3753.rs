pub fn total_waviness(num1: i64, num2: i64) -> i64 {
    let mut ans = 0;
    for n in num1..=num2 {
        ans+=count_waviness(n);
    }
    ans
}

fn count_waviness(mut n: i64) -> i64 {
    let mut count = 0;
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    if digits.len() <3{
        return 0;
    }
    for i in 1..digits.len()-1 {
        let pre = digits[i-1];
        let now = digits[i];
        let next = digits[i+1];
        if (pre < now && next < now) || (pre > now && next > now){
            count += 1;
        }
    }
    count
}