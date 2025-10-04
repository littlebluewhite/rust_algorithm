pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0usize, height.len()-1);
    let mut best = 0;
    while l < r {
        let min = height[l].min(height[r]);
        let area = min * (r - l) as i32;
        if area > best {
            best = area;
        }
        if height[l] >= height[r] {
            r -= 1;
        } else {
            l += 1;
        }
    }
    best
}
