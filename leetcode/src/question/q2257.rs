
pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (m as usize, n as usize);

    // 0 = empty, 1 = guard, 2 = wall, 3 = guarded
    let mut grid = vec![vec![0u8; n]; m];
    for wall in walls {
        grid[wall[0] as usize][wall[1] as usize] = 2;
    }
    for guard in &guards {
        grid[guard[0] as usize][guard[1] as usize] = 1;
    }
    let dist = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for guard in guards{
        let (r, c) = (guard[0], guard[1]);
        for (dr, dc) in dist {
            let mut nr = r+ dr;
            let mut nc = c+ dc;
            while nr >=0 && nc >=0 && nr < m as i32 && nc < n as i32{
                if grid[nr as usize][nc as usize] == 2 || grid[nr as usize][nc as usize] == 1 {
                    break;
                }
                grid[nr as usize][nc as usize] = 3;
                nr += dr;
                nc += dc;
            }
        }
    }
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                ans += 1;
            }
        }
    }
    ans
}