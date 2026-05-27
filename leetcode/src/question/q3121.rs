pub fn number_of_special_chars(word: String) -> i32 {
    let n = word.len() as i32;
    let mut last_lower = [-1i32;26];
    let mut first_upper = [n;26];
    for (i, &b) in word.as_bytes().iter().enumerate() {
        let i = i as i32;
        if b.is_ascii_lowercase() {
            let idx = (b - b'a') as usize;
            last_lower[idx] = i;
        } else {
            let idx = (b - b'A') as usize;
            first_upper[idx] = first_upper[idx].min(i);
        }
    }
    let mut count = 0;
    for i in 0..26{
        if first_upper[i] != n && last_lower[i] != -1 && first_upper[i]> last_lower[i] {
            count += 1;
        }
    }
    count
}