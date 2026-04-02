pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
    let n = lcp.len();
    let mut words: Vec<u8> = vec![0; n];
    let mut next_word = 1u8;
    for i in 0..n {
        if words[i] == 0 {
            if next_word > 26 {
                return String::new();
            }
            words[i] = next_word;
            next_word += 1;
        }
        for j in (i + 1)..n {
            if lcp[i][j] > 0 {
                if words[j] != words[i] && words[j] != 0 {
                    return String::new();
                }
                words[j] = words[i];
            }
        }
    }

    for i in (0..n).rev() {
        for j in (0..n).rev() {
            let expected = if words[i] == words[j] {
                if i + 1 == n || j + 1 == n {
                    1
                } else {
                    lcp[i + 1][j + 1] + 1
                }
            } else {
                0
            };
            if lcp[i][j] != expected {
                return String::new();
            }
        }
    }
    words.into_iter().map(|c| (c - 1 + b'a') as char).collect()
}
