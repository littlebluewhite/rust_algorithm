use std::collections::{ VecDeque};

pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
    let n = classroom.len();
    let m = classroom[0].len();
    let mut litter_id:Vec<i32> = vec![i32::MAX;m*n];
    let mut litter_count = 0;
    let mut start = 0;
    let mut new_classroom: Vec<Vec<u8>> = Vec::with_capacity(classroom.len());
    for i in 0..n {
        let bytes = classroom[i].as_bytes();
        for j in 0..m {
            if bytes[j] == b'S' {
                start = (i * m + j)as i32;
            } else if bytes[j] == b'L' {
                let id = i * m + j;
                litter_id[id] = litter_count;
                litter_count += 1;
            }
        }
        new_classroom.push(bytes.to_vec());
    }
    let full_mask = (1i32 << litter_count)-1;
    if full_mask == 0 {
        return 0;
    }
    let cells = (n * m) as i32;
    let energy_states = energy + 1;
    let full_state = (1i32 << litter_count)*cells* energy_states;
    let mut visited = vec![false; full_state as usize];
    let encode = |pos: i32, mask: i32, energy: i32|
        (mask* cells +pos)* energy_states + energy;
    let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut q = VecDeque::new();
    let start_state = encode(start, 0, energy);
    visited[start_state as usize] = true;
    q.push_back(start_state);
    let mut moves = 0;
    while !q.is_empty() {
        for _ in 0..q.len() {
            let state = q.pop_front().unwrap();
            let fuel = state % energy_states;
            let rest = state / energy_states;
            let pos = rest % cells;
            let mask = rest / cells;
            if fuel == 0 {
                continue;
            }

            let row = pos / m as i32;
            let col = pos % m as i32;
            for &(dr, dc) in dirs.iter() {
                let nr = row + dr;
                let nc = col + dc;
                if nr < 0 || nr >= n as i32 || nc < 0 || nc >= m as i32 {
                    continue;
                }

                let next_pos = nr  * m as i32+ nc ;
                let cell = new_classroom[nr as usize][nc as usize];
                if cell == b'X' {
                    continue;
                }

                let mut next_fuel = fuel - 1;
                let mut next_mask = mask;
                if cell == b'L' {
                    next_mask |= 1i32 << litter_id[next_pos as usize];
                }
                if cell == b'R' {
                    next_fuel = energy;
                }

                if next_mask == full_mask {
                    return moves + 1;
                }

                let next_state = encode(next_pos, next_mask, next_fuel);
                if !visited[next_state as usize] {
                    visited[next_state as usize] = true;
                    q.push_back(next_state);
                }
            }
        }
        moves += 1;
    }
    -1
}
