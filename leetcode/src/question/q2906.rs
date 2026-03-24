pub fn construct_product_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row = grid.len();
    let col = grid[0].len();
    const MOD: i64 = 12345;
    let mut result = vec![vec![1; col]; row];
    let mut prefix: i64 = 1;
    for i in 0..row{
        for j in 0..col{
            result[i][j] = prefix as i32;
            prefix = (prefix * grid[i][j] as i64)%MOD;
        }
    }
    let mut suffix: i64 = 1;
    for i in (0..row).rev(){
        for j in (0..col).rev(){
            result[i][j] = ((result[i][j] as i64* suffix) % MOD) as i32;
            suffix = (suffix * grid[i][j] as i64) % MOD;
        }
    }
    result
}