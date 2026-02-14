pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let r = query_row as usize;
    let g = query_glass as usize;

    let mut flow = vec![vec![0.0; r + 1]; r + 1];
    flow[0][0] = poured as f64;

    for row in 0..r {
        for col in 0..=row {
            let extra = (flow[row][col] - 1.0) / 2.0;
            if extra > 0.0 {
                flow[row + 1][col] += extra;
                flow[row + 1][col + 1] += extra;
            }
        }
    }
    println!("{:?}", flow);
    flow[r][g].min(1.0)
}
