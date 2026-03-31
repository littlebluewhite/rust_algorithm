pub fn check_strings(s1: String, s2: String) -> bool {
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let n = b1.len();
    let mut diff = vec![vec![0; 26]; 2];
    for (i, (&c1, &c2)) in b1.iter().zip(b2.iter()).enumerate() {
        let parity = i & 1;
        diff[parity][(c1 - b'a') as usize] += 1;
        diff[parity][(c2 - b'a') as usize] -= 1;
    }
    diff.iter().all(|d| d.iter().all(|&v| v == 0))
}
