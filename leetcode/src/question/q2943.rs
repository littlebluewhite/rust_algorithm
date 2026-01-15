pub fn maximize_square_hole_area(
    n: i32,
    m: i32,
    mut h_bars: Vec<i32>,
    mut v_bars: Vec<i32>,
) -> i32 {
    let h_max = find_sequence(h_bars);
    let v_max = find_sequence(v_bars);
    let l = h_max.min(v_max) + 1;
    l * l
}

fn find_sequence(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut max = 0;
    let mut now = 0;
    let mut pre = 0;
    for i in 0..nums.len() {
        if nums[i] == pre + 1 {
            now += 1;
        } else {
            if now > max {
                max = now;
            }
            now = 1;
        }
        pre = nums[i];
    }
    if now > max {
        max = now;
    }
    max
}
