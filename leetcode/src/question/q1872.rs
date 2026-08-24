pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let mut prefix = vec![0; n];
    prefix[0] = stones[0];
    for i in 1..n {
        prefix[i] = prefix[i-1] + stones[i];
    }
    let mut best = prefix[n-1];
    for i in (1..n-1).rev() {
        best = best.max(prefix[i]-best);
    }
    best
}