pub fn lex_greater_permutation(s: String, target: String) -> String {
    let mut freq = [0; 26];
    let n = s.len();
    for c in s.chars() {
        freq[(c as u8 - b'a') as usize] += 1;
    }
    let target_bytes = target.as_bytes();
    let mut matched = 0usize;
    for i in 0..n{
        let c = target_bytes[i];
        if freq[(c - b'a') as usize] > 0 {
            matched += 1;
            freq[(c - b'a') as usize] -= 1;
        }else{
            break;
        }
    }
    if matched == n {
        matched = n-1;
        freq[(target_bytes[n-1] - b'a')as usize] += 1;
    };
    loop {
        let pilot = target_bytes[matched]-b'a';
        for bigger in (pilot + 1)..26 {
            if freq[bigger as usize] == 0 {
                continue;
            }
            let mut ans = target_bytes[..matched].to_vec();
            ans.push(bigger + b'a');
            freq[bigger as usize] -= 1;
            for c in 0..26 {
                let count = freq[c];
                for _ in 0..count {
                    ans.push(c as u8 + b'a');
                }
            }
            return String::from_utf8(ans).unwrap();
        }
        if matched == 0{
            break;
        }
        matched -= 1;
        freq[(target_bytes[matched] - b'a') as usize] += 1;
    }
    "".to_string()
}