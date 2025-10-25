pub fn total_money(n: i32) -> i32 {
    let mut count = 0;
    let week = n/7;
    for i in 0..week{
        count += 28+7*i
    }
    let day = n%7;
    for i in 0..day{
        count += 1+i+week
    }
    count
}