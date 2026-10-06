pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut open = 0;
    let mut add_open = 0;
    for &u in s.as_bytes() {
        if u == b'('{
            open += 1;
        }else if open <= 0{
            add_open += 1;
        }else{
            open -= 1;
        }
    }
    add_open + open
}