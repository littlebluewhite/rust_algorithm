pub fn has_same_digits(s: String) -> bool {
    let mut digits: Vec<u8> = s.chars().map(|c| c as u8 - b'0').collect();
    while digits.len() > 2 {
        let mut next = Vec::with_capacity(digits.len() - 1);
        for i in 0..digits.len() - 1 {
            next.push((digits[i] + digits[i + 1]) % 10);
        }
        digits = next;
    }
    digits[0] == digits[1]
}
