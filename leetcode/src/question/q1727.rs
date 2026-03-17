pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut max_area = 0;
    for i in 0..n{
        for j in 0..m{
            if matrix[i][j] ==1 && i>0{
                matrix[i][j] = matrix[i-1][j] + 1;
            }
        }
        let mut height = matrix[i].clone();
        height.sort_by(|a, b| b.cmp(a));
        for j in 0..m{
            if height[j] > 0{
                let area = height[j] * (j+1) as i32;
                max_area = max_area.max(area);
            }
        }
    }
    max_area
}