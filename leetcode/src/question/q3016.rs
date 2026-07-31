pub fn minimum_pushes(word: String) -> i32 {
    let mut freq = [0i32;26];
    for b in word.as_bytes(){
        freq[(b - b'a') as usize] += 1;
    }
    freq.sort_unstable();
    freq.reverse();
    let mut ans = 0;
    for i in 0..26{
        ans += freq[i] * (i as i32 / 8 + 1);
    }
    ans
}