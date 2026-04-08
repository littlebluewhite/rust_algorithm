use std::collections::HashSet;

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let obstacles_set: HashSet<(i32, i32)> = obstacles.into_iter().map(|v| (v[0], v[1])).collect();
    let dist = [(0,1), (1,0), (0,-1), (-1,0)];
    let mut dir = 0;
    let mut best = 0;
    let mut pos = (0,0);
    for com in commands {
        match com{
            -1 => dir = (dir + 1) % 4,
            -2 => dir = (dir + 3) % 4,
            step => {
                for _ in 0..step{
                    let next_x = pos.0 + dist[dir].0;
                    let next_y = pos.1 + dist[dir].1;
                    if obstacles_set.contains(&(next_x, next_y)) {
                        break;
                    }
                    pos = (next_x, next_y);
                    best = best.max(pos.0*pos.0 + pos.1*pos.1)
                }
            }
        }
    }
    best
}