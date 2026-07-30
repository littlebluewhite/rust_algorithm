pub fn minimum_pushes(word: String) -> i32 {
    let mut n = word.len() as i32;
    let mut ans = n;
    while n > 8{
        n -= 8;
        ans += n;
    }
    ans
}