pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let n = words.len();
    let mut ans = i32::MAX;
    for (i, word) in words.iter().enumerate(){
        if word == &target{
            let mut diff = (i as i32 -start_index).abs();
            diff = diff.min(n as i32 - diff);
            ans = ans.min(diff);
        }
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}