pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
    let mut best = 0i64;
    let n = bottom_left.len();
    for i in 0..n-1{
        for j in i+1..n{
            let left = bottom_left[i][0].max(bottom_left[j][0]);
            let right = top_right[i][0].min(top_right[j][0]);
            let bottom = bottom_left[i][1].max(bottom_left[j][1]);
            let top = top_right[i][1].min(top_right[j][1]);
            let w = right-left;
            let h = top-bottom;
            if w>0 &&h>0{
                let side = (w).min(h);
                let area = side as i64 * side as i64;
                if area > best{
                    best = area
                }
            }
        }
    }
    best
}