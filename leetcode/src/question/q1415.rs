pub fn get_happy_string(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize;
    let total = 3usize << (n - 1);
    let mut ans: Vec<u8> = Vec::with_capacity(n);
    if k <= 0 || k > total {
        return String::new();
    }
    let mut index = k - 1;
    for i in 0..n {
        let block = 1 << (n - 1 - i);
        for &ch in b"abc" {
            if Some(&ch) == ans.last() {
                continue;
            } else {
                if index >= block {
                    index -= block;
                } else {
                    ans.push(ch);
                    break;
                }
            }
        }
    }
    String::from_utf8(ans).unwrap()
}
