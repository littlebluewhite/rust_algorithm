pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
    capacity.sort_by(|a, b| b.cmp(a));
    let total: i32 = apple.iter().sum();
    let mut count_total = 0;
    for i in 0..capacity.len(){
        count_total += capacity[i];
        if count_total >= total {
            return i as i32+1;
        }
    }
    0
}