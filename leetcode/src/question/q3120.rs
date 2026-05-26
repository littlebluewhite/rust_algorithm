pub fn number_of_special_chars(word: String) -> i32{
    let mut lower = 0u32;
    let mut upper = 0u32;
    for b in word.as_bytes(){
        if b.is_ascii_lowercase(){
            lower |= 1 << (b - b'a')
        }else if b.is_ascii_uppercase(){
            upper |= 1 << (b - b'A')
        }
    }
    (lower & upper).count_ones() as i32
}