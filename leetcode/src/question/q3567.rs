pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let n = grid.len();
    let m = grid[0].len();
    let k = k as usize;
    if k < 2{
        return vec![vec![0;m];n]
    }
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..=n-k{
        let mut row = Vec::with_capacity(m-k+1);
        for j in 0..=m-k{
            let mut v = Vec::with_capacity(k*k);
            for a in i..i+k{
                for b in j..j+k{
                    v.push(grid[a][b]);
                }
            }
            let min = get_min_abs(v);
            row.push(min)
        }
        ans.push(row);
    }
    ans
}

fn get_min_abs(mut v: Vec<i32>) -> i32{
    v.sort_unstable();
    v.dedup();
    if v.len() == 1 {
        return 0
    }
    let mut min = i32::MAX;
    for i in 1..v.len(){
        min = min.min(v[i]-v[i-1])
    }
    min
}