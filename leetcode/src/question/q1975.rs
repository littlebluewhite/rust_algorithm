pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut ans = 0i64;
    let mut min = i32::MAX;
    let mut negative_count = 0i32;
    for row in matrix {
        for val in row {
            let mut positive_val = val;
            if val < 0 {
                positive_val = -val;
                negative_count += 1;
            }
            if positive_val<min{
                min = positive_val
            }
            ans += positive_val as i64;
        }
    }
    if negative_count % 2 != 0 {
        ans -= min as i64 * 2;
    }
    ans
}
