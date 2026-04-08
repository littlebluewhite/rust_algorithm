pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    let rows = rows as usize;
    let text = encoded_text.as_bytes();
    if rows == 1 || encoded_text.is_empty(){
        return encoded_text;
    }
    let n = text.len();
    let cols = n / rows as usize;
    let mut result_bytes: Vec<u8> = Vec::with_capacity(n);
    for start_c in 0..cols{
        let mut c = start_c;
        let mut r = 0;
        while c < cols && r < rows{
            result_bytes.push(text[c+r*cols]);
            c += 1;
            r += 1;
        }
    }
    while result_bytes.last() == Some(&b' '){
        result_bytes.pop();
    }
    String::from_utf8(result_bytes).unwrap()
}