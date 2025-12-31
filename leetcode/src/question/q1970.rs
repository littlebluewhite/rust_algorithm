use std::collections::VecDeque;

pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let mut l = 0;
    let mut r = cells.len();
    while l < r {
        let m = (l + r + 1) / 2;
        println!("{} {} {}", l, m, r);
        let can = can_cross(row, col, &cells, m);
        if can {
            l = m;
        } else {
            r = m - 1;
        }
    }
    l as i32
}

fn can_cross(row: i32, col: i32, cells: &Vec<Vec<i32>>, day: usize) -> bool {
    let mut grid = vec![vec![0; col as usize]; row as usize];
    for i in 0..day {
        grid[(cells[i][0] - 1) as usize][(cells[i][1] - 1) as usize] = 1;
    }
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    for j in 0..col {
        if grid[0][j as usize] == 0 {
            queue.push_back((0, j));
            grid[0][j as usize] = 1;
        }
    }
    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in dirs {
            let nr = r + dr;
            let nc = c + dc;
            if nr < 0 || nr >= row || nc < 0 || nc >= col {
                continue;
            }
            if grid[nr as usize][nc as usize] == 1 {
                continue;
            }
            if nr == row - 1 {
                return true;
            }
            grid[nr as usize][nc as usize] = 1;
            queue.push_back((nr, nc));
        }
    }
    false
}
