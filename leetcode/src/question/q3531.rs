use std::collections::HashMap;

pub fn count_covered_buildings(_n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    let mut x_map: HashMap<i32, (i32, i32)> = HashMap::new();
    let mut y_map: HashMap<i32, (i32, i32)> = HashMap::new();
    for building in &buildings {
        let (x, y) = (building[0], building[1]);
        x_map.entry(x).and_modify(|(min_y, max_y)| {
            *min_y = (*min_y).min(y);
            *max_y = (*max_y).max(y);
        }).or_insert((y, y));
        y_map.entry(y).and_modify(|(min_x, max_x)| {
            *min_x = (*min_x).min(x);
            *max_x = (*max_x).max(x);
        }).or_insert((x, x));
    }
    let mut ans = 0i32;
    for building in &buildings {
        let (x, y) = (building[0], building[1]);
        let (min_y, max_y) = x_map.get(&x).unwrap();
        let (min_x, max_x) = y_map.get(&y).unwrap();
        if min_x < &x && max_x > &x && min_y < &y && max_y > &y {
            ans += 1;
        }
    }
    ans
}