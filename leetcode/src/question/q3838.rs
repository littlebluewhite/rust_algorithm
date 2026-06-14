pub fn map_word_mapping(words: Vec<String>, weights: Vec<i32>) -> String {
    let mut ans = String::with_capacity(words.len());
    for word in words {
        let mut sum = 0;
        for &b in word.as_bytes() {
            let idx = (b-b'a') as usize;
            sum += weights[idx];
        }

        let r = (sum % 26) as u8;
        ans.push((b'z'-r) as char);
    }
    ans
}