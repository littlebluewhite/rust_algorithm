pub fn min_changes(s: String) -> i32 {
    let mut c = 0;
    for pair in s.as_bytes().chunks(2) {
        if pair[0] != pair[1] {
            c += 1;
        }
    }
    c
}