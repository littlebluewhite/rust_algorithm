pub fn sum_game(num: String) -> bool {
    let b = num.as_bytes();
    let n = b.len();
    let half = n / 2;
    let mut question_total = 0;
    let mut question_delta = 0;
    let mut diff = 0;
    for i in 0..half {
        if b[i] == b'?' {
            question_total += 1;
            question_delta -= 1;
        } else {
            diff += (b[i] - b'0') as i32;
        }
    }
    for i in half..n {
        if b[i] == b'?' {
            question_total += 1;
            question_delta += 1;
        } else {
            diff -= (b[i] - b'0') as i32;
        }
    }
    if question_total % 2 == 1 {
        return true;
    }
    diff != question_delta * 9 / 2
}
