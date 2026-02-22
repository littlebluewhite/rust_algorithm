pub fn binary_gap(n: i32) -> i32 {
    let mut n = n as u32;
    let mut position = 0;
    let mut last_one: Option<i32> = None;
    let mut ans = 0;
    while n > 0{
        if (n & 1) == 1{
            if let Some(prev) = last_one{
                ans = ans.max(position-prev);
            }
            last_one = Some(position);
        }
        position += 1;
        n >>= 1;
    }
    ans
}