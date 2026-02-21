pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut ans = 0;
    for i in left..=right{
        let i = i as u32;
        if is_prime(i.count_ones()){
            ans += 1;
        }
    }
    ans
}

fn is_prime(n: u32) -> bool{
    if n < 2{
        return false
    }
    if n == 2{
        return true
    }
    if n %2==0{
        return false
    }
    let mut d = 3;
    while d*d <= n{
        if n%d == 0{
            return false
        }
        d+=2;
    }
    true
}