pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
    if num * 9 < sum {
        return "".to_string();
    }
    let mut result = String::with_capacity(num as usize);
    let mut remaining = sum;
    for _ in 0..num {
        let digit = remaining.min(9);
        remaining -= digit;
        result.push(char::from(b'0' + digit as u8));
    }
    result
}
