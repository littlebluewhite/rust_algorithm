pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    const NEG:i64 = i64::MIN/4;
    let mut rest = vec![NEG; k+1];
    let mut long = vec![NEG; k+1];
    let mut short = vec![NEG; k+1];
    rest[0] = 0;
    for price in prices{
        let price = price as i64;
        let mut next_rest = rest.clone();
        let mut next_long = long.clone();
        let mut next_short = short.clone();
        for t in 0..=k{
            if t < k{
                if long[t] != NEG{
                    next_rest[t+1] = rest[t].max(long[t]+price);
                }
                if short[t] != NEG{
                    next_rest[t+1] = rest[t].max(short[t]-price);
                }
            }
            if rest[t] != NEG{
                next_long[t] = long[t].max(rest[t]-price);
                next_short[t] = short[t].max(long[t]+price);
            }
        }

    }
    0
}