pub fn mirror_distance(n: i32) -> i32 {
    let mut r = reverse(n);
    (r-n).abs()
}

fn reverse(mut a: i32) -> i32{
    let mut r = 0;
    while a > 0 {
        r = r * 10 + a % 10;
        a /= 10;
    }
    r
}