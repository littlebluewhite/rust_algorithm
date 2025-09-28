pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let n = points.len();
    let mut best = 0f64;
    for i in 0..n{
        for j in (i+1)..n{
            for k in (j+1)..n{
                let (x1, y1) = (points[i][0] as i64, points[i][1] as i64);
                let (x2, y2) = (points[j][0] as i64, points[j][1] as i64);
                let (x3, y3) = (points[k][0] as i64, points[k][1] as i64);

                // 「兩倍面積」
                let area = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() as f64;

                if area > best {
                    best = area;
                }
            }

        }
    }
    best/2.0
}