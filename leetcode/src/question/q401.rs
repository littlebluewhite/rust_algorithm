pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let target = turned_on as u32;
    let mut ans = Vec::new();

    for hour in 0..12u32 {
        for minute in 0..60u32 {
            if hour.count_ones() + minute.count_ones() == target {
                ans.push(format!("{}:{:02}", hour, minute));
            }
        }
    }

    ans
}