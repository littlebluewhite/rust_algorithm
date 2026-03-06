pub fn check_ones_segment(s: String) -> bool {
    let mut check = false;
    for c in s.bytes() {
        if check == true {
            if c == b'1' {
                return false;
            }
        } else {
            if c == b'0' {
                check = true;
            }
        }
    }
    true
}
