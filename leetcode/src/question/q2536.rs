pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut diff = vec![vec![0i32; n + 1]; n + 1];
    for q in queries {
        let (left_i, left_j) = (q[0] as usize, q[1] as usize);
        let (right_i, right_j) = (q[2] as usize + 1, q[3] as usize + 1);
        diff[left_i][left_j] += 1;
        diff[right_i][left_j] -= 1;
        diff[left_i][right_j] -= 1;
        diff[right_i][right_j] += 1;
    }
    println!("{:?}", diff);

    let mut ans = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            let up = if i > 0 { ans[i - 1][j]} else { 0 };
            let left = if j > 0 { ans[i][j - 1]} else { 0 };
            let diag = if i > 0 && j > 0 { ans[i - 1][j - 1] } else { 0 };
            ans[i][j] = up + left + diff[i][j] - diag;
        }
    }
    ans
}
