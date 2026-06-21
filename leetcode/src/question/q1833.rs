pub fn max_ice_cream(mut costs: Vec<i32>,mut coins: i32) -> i32 {
    costs.sort_unstable();
    let n = costs.len();
    let mut ans = 0;
    for i in 0..n{
        if coins >= costs[i]{
            coins -= costs[i];
            ans += 1;
        }else{
            break
        }
    }
    ans
}