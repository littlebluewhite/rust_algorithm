use std::collections::HashMap;

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a.abs()
    }
    fn normalize(dx: i64, dy: i64) -> (i32, i32) {
        let g = gcd(dx.abs(), dy.abs());
        let mut dx = dx / g;
        let mut dy = dy / g;
        if dx < 0 || (dx == 0 && dy < 0) {
            dx = -dx;
            dy = -dy;
        }
        (dx as i32, dy as i32)
    }
    let pts: Vec<(i64, i64)> = points.iter().map(|p| (p[0] as i64, p[1] as i64)).collect();
    let mut lines: HashMap<(i32, i32), HashMap<i64, Vec<usize>>> = HashMap::new();
    for i in 0..n {
        for j in i + 1..n {
            let (dx, dy) = normalize(pts[i].0 - pts[j].0, pts[i].1 - pts[j].1);
            let line = lines.entry((dx, dy)).or_insert(HashMap::new());
            let line = line
                .entry((points[i][0] as i64 * dy as i64 - points[i][1] as i64 * dx as i64) as i64)
                .or_insert(Vec::new());
            line.push(i);
            line.push(j);
        }
    }
    for slope_map in lines.values_mut() {
        for pts_on_line in slope_map.values_mut() {
            pts_on_line.sort_unstable();
            pts_on_line.dedup();
        }
    }

    let mut total_parallel = 0i64;
    for slope_map in lines.values() {
        let mut sum = 0i64;
        let mut sum_b = 0i64;
        for pts_on_line in slope_map.values() {
            let cnt = pts_on_line.len() as i64;
            if cnt < 2 {
                continue;
            }
            let b = cnt * (cnt - 1) / 2;
            sum += b;
            sum_b += b * b;
        }
        total_parallel += (sum * sum - sum_b) / 2;
    }

    let mut midpoint_map: HashMap<(i64, i64), Vec<(i32, i32)>> = HashMap::new();
    for i in 0..n {
        for j in i + 1..n {
            let dir = normalize(pts[i].0 - pts[j].0, pts[i].1 - pts[j].1);
            let mid = (pts[i].0 + pts[j].0, pts[i].1 + pts[j].1);
            midpoint_map.entry(mid).or_default().push(dir);
        }
    }
    let mut parallelograms = 0i64;
    for dir in midpoint_map.values() {
        let mut check_dir: HashMap<(i32, i32), i64> = HashMap::new();
        let cnt = dir.len() as i64;
        if cnt < 2 {
            continue;
        }
        for &d in dir {
            *check_dir.entry(d).or_default() += 1;
        }
        let mut total_pairs = cnt * (cnt - 1) / 2;
        for &nums in check_dir.values() {
            if nums >= 2 {
                total_pairs -= nums * (nums - 1) / 2;
            }
        }
        parallelograms += total_pairs
    }
    (total_parallel - parallelograms) as i32
}
