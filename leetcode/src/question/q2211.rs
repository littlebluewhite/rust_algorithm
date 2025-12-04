pub fn count_collisions(directions: String) -> i32 {
    let n = directions.len();
    let directions = directions.as_bytes();
    let mut l = 0usize;
    let mut r = n;
    while l < r && directions[l] == b'L'{
        l += 1;
    }
    while l < r && directions[r-1] == b'R'{
        r -= 1;
    }
    let mut ans = 0;
    for &ch in &directions[l..r]{
        if ch != b'S'{
            ans += 1;
        }
    }
    ans
}