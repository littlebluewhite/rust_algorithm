pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let mut dp1 = 0;
    let mut dp2 = 0;
    let mut dp3 = 0;
    let n = stone_value.len();
    for i in (0..n).rev() {
        let mut value = 0;
        let mut best = i32::MIN;
        for take in 1..=3{
            if i + take > n {
                break;
            }
            value += stone_value[i + take - 1];
            let diff = match take {
                1 => dp1,
                2 => dp2,
                3 => dp3,
                _ => unreachable!(),
            };
            best = best.max(value - diff);
        }
        dp3 = dp2;
        dp2 = dp1;
        dp1 = best;
    }
    if dp1 > 0{
        "Alice".to_string()
    } else if dp1 < 0 {
        "Bob".to_string()
    } else {
        "Tie".to_string()
    }
}
