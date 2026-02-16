pub fn add_binary(a: String, b: String) -> String {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut i = ab.len() as i32 - 1;
    let mut j = bb.len() as i32 - 1;
    let mut carry = 0;
    let mut out: Vec<u8> = Vec::with_capacity(ab.len().max(bb.len()) + 1);

    while i >= 0 || j >= 0 || carry > 0 {
        let da = if i >= 0 {
            (ab[i as usize] - b'0') as i32
        } else {
            0
        };
        let db = if j >= 0 {
            (bb[j as usize] - b'0') as i32
        } else {
            0
        };
        let sum = da + db + carry;

        out.push((sum % 2) as u8 + b'0');
        carry = sum / 2;
        i -= 1;
        j -= 1;
    }

    out.reverse();
    String::from_utf8(out).unwrap()
}
