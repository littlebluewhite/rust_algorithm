pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    const NEG:i64 = i64::MIN/4;
    let mut rest = vec![NEG; k+1];
    let mut long = vec![NEG; k+1];
    let mut short = vec![NEG; k+1];
    rest[0] = 0;
    for price in prices{
        let mut rest_next = rest.clone();
        let mut long_next = long.clone();
        let mut short_next = short.clone();
        for r in 0..k{
            if long[r] != NEG{
                rest_next[r+1] = rest_next[r+1].max(long[r]+price as i64);
            }
            if short[r] != NEG{
                rest_next[r+1] = rest_next[r+1].max(short[r]-price as i64);
            }
            if rest[r] != NEG{
                long_next[r] = long[r].max(rest[r] - price as i64);
                short_next[r] = short[r].max(rest[r] + price as i64);
            }
        }
        rest = rest_next;
        long = long_next;
        short = short_next;
    }
    *rest.iter().max().unwrap()
}