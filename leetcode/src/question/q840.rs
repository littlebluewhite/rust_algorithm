pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    if n < 3 || m < 3 {
        return 0;
    }
    let mut ans = 0i32;
    for i in 0..n - 2 {
        for j in 0..m - 2 {
            if is_magic(i, j, &grid) {
                ans += 1;
            }
        }
    }
    ans
}

fn is_magic(h: usize, v: usize, grid: &Vec<Vec<i32>>) -> bool {
    let mut number_map = vec![false; 9];
    let mut diagonals_1 = 0i32;
    let mut diagonals_2 = 0i32;
    let mut first_i_total = 0i32;
    let mut second_i_total = 0i32;
    let mut third_i_total = 0i32;
    for (i_index, i) in (h..h + 3).enumerate() {
        let mut j_total = 0i32;
        for (j_index, j) in (v..v + 3).enumerate() {
            let val = grid[i][j];
            if val < 1 || val > 9 || number_map[(val - 1) as usize] == true {
                return false;
            } else {
                number_map[(val - 1) as usize] = true;
            }
            match j_index {
                0 => first_i_total += val,
                1 => second_i_total += val,
                2 => third_i_total += val,
                _ => {}
            }
            j_total += val;
            match (i_index, j_index) {
                (0, 0) => {
                    diagonals_1 += val;
                }
                (0, 2) => {
                    diagonals_2 += val;
                }
                (1, 1) => {
                    diagonals_1 += val;
                    diagonals_2 += val;
                }
                (2, 0) => {
                    diagonals_2 += val;
                }
                (2, 2) => {
                    diagonals_1 += val;
                }
                _ => {}
            }
        }
        if j_total != 15 {
            return false;
        }
    }
    if first_i_total != 15
        || second_i_total != 15
        || third_i_total != 15
        || diagonals_1 != 15
        || diagonals_2 != 15
    {
        return false;
    }
    true
}
