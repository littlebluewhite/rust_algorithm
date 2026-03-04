use std::collections::HashMap;

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let m = mat[0].len();
    let mut column_map: HashMap<usize, usize> = HashMap::new();
    let mut row_map: HashMap<usize, usize> = HashMap::new();
    let mut count = 0;
    for i in 0..n{
        for j in 0..m{
            if mat[i][j] == 1{
                column_map.entry(j).and_modify(|v| *v += 1).or_insert(1);
                row_map.entry(i).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    for i in 0..n{
        for j in 0..m{
            if mat[i][j] == 1 && column_map[&j] == 1 && row_map[&i] == 1{
                count += 1;
            }
        }
    }
    count
}