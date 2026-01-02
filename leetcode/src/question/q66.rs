pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in 0..digits.len() {
        if digits[i] < 9{
            digits[i] += 1;
            return digits
        }
        digits[i] = 0;
    }
    let mut res = Vec::with_capacity(digits.len() + 1);
    res.push(1);
    res.extend(digits);
    res
}