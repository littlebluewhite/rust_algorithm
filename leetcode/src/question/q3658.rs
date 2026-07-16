pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    let mut odd_sum = 0;
    let mut even_sum = 0;
    for i in 1..=2*n{
        if i % 2 == 0{
            even_sum += i;
        }else{
            odd_sum += i;
        }
    }
    gcd(odd_sum, even_sum)
}

fn gcd(mut a: i32,mut b: i32) -> i32{
    if b<a{
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0{
        let r = a%b;
        a = b;
        b = r;
    }
    a
}