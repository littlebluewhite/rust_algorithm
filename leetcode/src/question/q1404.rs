pub fn num_steps(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut step = 0;
    let mut carry: i32 = 0;
    for i in (1..n).rev() {
        let bit = (b[i] - b'0') as i32 + carry;
        if bit == 1 {
            carry = 1;
            step += 2;
        } else {
            step += 1;
        }
    }
    step + carry
}