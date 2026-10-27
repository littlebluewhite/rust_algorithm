pub fn create_grid(m: i32, n: i32) -> Vec<String> {
    let mut grid = Vec::with_capacity(m as usize);
    for i in 0..m {
        let mut row = String::with_capacity(n as usize);
        for j in 0..n {
            if i == 0{
                row.push('.');
            }else if j==n-1{
                row.push('.');
            }else{
                row.push('#');
            }
        }
        grid.push(row);
    }
    grid
}