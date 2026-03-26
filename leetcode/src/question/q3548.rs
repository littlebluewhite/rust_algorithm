use std::collections::HashSet;

pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
    for _ in 0..4{
        if check(&grid){
            return true;
        }
        grid = rotate(grid);
    }
    false
}

fn rotate(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut result = vec![vec![0;n];m];
    for i in 0..n{
        for j in 0..m{
            result[j][n - 1 - i] = grid[i][j];
        }
    }
    result
}

fn check(grid: &Vec<Vec<i32>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    if n == 1{
        return false
    }
    let total = {
        let mut sum = 0i64;
        for i in 0..n{
            for j in 0..m{
                sum += grid[i][j] as i64;
            }
        }
        sum
    };
    let mut upper = 0i64;
    let mut set: HashSet<i64> = HashSet::new();
    for i in 0..n-1{
        for j in 0..m{
            upper += grid[i][j] as i64;
            set.insert(grid[i][j] as i64);
        }
        let need = upper *2 - total;
        if m == 1{
            if need == 0 || need == grid[0][0] as i64 || grid[i][0] as i64 == need{
                return true;
            }
        }else if i == 0{
            if need == 0 || need == grid[0][0] as i64 || grid[0][m-1] as i64 == need{
                return true;
            }
        }else{
            if need == 0 || set.contains(&need){
                return true;
            }
        }
    }
    false
}