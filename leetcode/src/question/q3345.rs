pub fn smallest_number(n: i32, t: i32) -> i32 {
    let mut res = n;
    loop {
        if digit_product(res) % t == 0{
            return res;
        }
        res += 1;
    }
}

fn digit_product(mut x: i32) -> i32{
    let mut product = 1;
    while x > 0{
        product *= x%10;
        x /= 10;
    }
    product
}