pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let points: Vec<(i32, i32)> = points.iter().map(|p| (p[0], p[1])).collect();
    let mut pre = points[0];
    let mut ans = 0;
    for i in 1..n{
        let(x, y) = points[i];
        let (px, py) = pre;
        ans += (px-x).abs().max((py-y).abs());
        pre = points[i];
    }
    ans
}