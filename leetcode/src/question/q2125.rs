pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut prev = 0i32;
    let mut ans = 0i32;
    for row in bank{
        let cnt = row.chars().filter(|c| *c == '1').count() as i32;
        if cnt == 0 {
            continue;
        }
        ans += prev * cnt;
        prev = cnt;
    }
    ans
}