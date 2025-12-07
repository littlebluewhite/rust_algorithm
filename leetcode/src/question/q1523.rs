pub fn count_odds(low: i32, high: i32) -> i32 {
    let mut ans = 0;
    if low % 2 == 1{
        ans = 1 + (high-low)/2
    }else{
        ans = (high-low+1)/2
    }
    ans
}
