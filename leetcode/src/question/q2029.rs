pub fn stone_game_ix(stones: Vec<i32>) -> bool {
    let mut cnt = [0i32; 3];
    for &x in &stones {
        cnt[(x as usize) % 3] += 1;
    }
    if cnt[0] % 2 == 0 {
        cnt[1] > 0 && cnt[2] > 0
    } else {
        (cnt[1] - cnt[2]).abs() > 2
    }
}
