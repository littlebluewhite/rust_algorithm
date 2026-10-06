pub fn check_valid_string(s: String) -> bool {
    let (mut high, mut low) = (0, 0);
    for &u in s.as_bytes() {
        if u == b'(' {
            high += 1;
            low += 1;
        } else if u == b')' {
            high -= 1;
            low -= 1;
        } else if u == b'*' {
            high += 1;
            low -= 1;
        }
        if high < 0 {
            return false;
        }
        low = low.max(0);
    }
    low == 0
}
