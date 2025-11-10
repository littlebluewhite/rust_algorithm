pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let n = stations.len();
    let r = r as usize;
    let k = k as i64;
    let mut pre = vec![0i64;n+1];
    for i in 0..n{
        pre[i+1] = pre[i] + stations[i] as i64;
    }

    let mut base = vec![0i64;n];
    for i in 0..n{
        let left = i.saturating_sub(r);
        let right = (i+r+1).min(n);
        base[i] = pre[right] - pre[left];
    }

    let can = |target: i64| -> bool{
        let mut add_win = 0i64;
        let mut used = 0;
        let mut add = vec![0;n];
        for i in 0..n{
            if i >= r+1{
                add_win -= add[i-r-1];
            }
            if add_win + base[i] < target{
                let need = target - (add_win + base[i]);
                if used+ need > k{
                    return false;
                }
                used += need;
                let index = (i+r).min(n-1);
                add[index] += need;
                add_win += need;
            }
        }
        true
    };
    let mut left = 0i64;
    let mut right = pre[n] + k;
    while left < right{
        let mid = (left +right+1)/2;
        if can(mid){
            left = mid
        }else{
            right = mid-1
        }
    }
    left
}