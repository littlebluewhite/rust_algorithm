pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
    const INF:i32 = i32::MAX/2;
    let (mut pre1, mut pre2, mut pre3) = (0, INF, INF);
    for i in 1..=n as usize{
        let cur = costs[i-1]+ (pre1+1).min((pre2+4)).min((pre3+9));
        (pre1, pre2, pre3) = (cur, pre1, pre2);
    }
    pre1
}