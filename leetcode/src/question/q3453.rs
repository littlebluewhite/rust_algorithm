pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let mut events: Vec<(i64, i64)> = Vec::new();
    let mut slope = 0i64;
    let mut total = 0f64;
    let mut area = 0f64;
    for s in squares {
        let y = s[1] as i64;
        let delta = s[2] as i64;
        events.push((y, delta));
        events.push((y + delta, -delta));
        total += (delta * delta) as f64;
    }
    events.sort_by_key(|e| e.0);
    let n = events.len();
    let target: f64 = total / 2.0;
    let mut i = 0;
    let mut prev_y = events[0].0;
    while i < n && events[i].0 == prev_y {
        slope += events[i].1;
        i += 1;
    }
    while i < n {
        let next_area = area + (slope * (events[i].0 - prev_y)) as f64;
        if target <= next_area {
            return prev_y as f64 + (target - area) / slope as f64;
        } else {
            area = next_area;
            prev_y = events[i].0;
            while i < n && events[i].0 == prev_y {
                slope += events[i].1;
                i += 1;
            }
        }
    }
    prev_y as f64
}
