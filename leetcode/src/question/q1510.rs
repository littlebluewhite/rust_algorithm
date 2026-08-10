pub fn winner_square_game(n: i32) -> bool {
    let mut dp = vec![false; n as usize+1];
    for i in 1..=n{
        let num = i;
        let mut base = 1;
        while base*base <= num{
            let square = base*base;
            if dp[(num-square) as usize] == false {
                dp[i as usize] = true;
                break;
            }
            base += 1;
        }
    }
    dp[n as usize]
}