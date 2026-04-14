fn dist(a: usize, b: usize) -> i32 {
    let (x1, y1) = ((a / 6) as i32, (a % 6) as i32);
    let (x2, y2) = ((b / 6) as i32, (b % 6) as i32);
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn minimum_distance(word: String) -> i32 {
    let letters: Vec<usize> = word.bytes().map(|c| (c - b'A') as usize).collect();
    let mut dp = vec![0; 26];
    for i in 1..letters.len() {
        let pre = letters[i - 1];
        let cur = letters[i];
        let d = dist(pre, cur);
        let mut next = vec![i32::MAX; 26];
        for other in 0..26 {
            next[other] = next[other].min(dp[other] + d);
            next[pre] = next[pre].min(dp[other] + dist(other, cur));
        }
        dp = next;
    }
    *dp.iter().min().unwrap()
}
