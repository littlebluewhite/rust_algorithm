fn dist(a: usize, b: usize) -> i32{
    let (ax, ay) = ((a / 6)as i32, (a % 6)as i32);
    let (bx, by) = ((b / 6)as i32, (b % 6) as i32);
    (ax - bx).abs() + (ay - by).abs()
}
pub fn minimum_distance(word: String) -> i32 {
    let letters: Vec<usize> = word.bytes().map(|c| (c - b'A') as usize).collect();
    let mut dp = vec![0; 26];
    for i in 1..letters.len(){
        let pre = letters[i-1];
        let cur = letters[i];
        let d = dist(cur, pre);
        let mut next = vec![i32::MAX;26];

        for other in 0..26{
            next[other] = next[other].min(dp[other] + d);
            next[pre] = next[pre].min(dp[other]+dist(other, cur));
        }
        dp = next;
    }
    *dp.iter().min().unwrap()
}
