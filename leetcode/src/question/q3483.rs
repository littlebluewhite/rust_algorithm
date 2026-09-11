pub fn total_numbers(digits: Vec<i32>) -> i32 {
    let n = digits.len() as i32;
    let mut freq = [0; 10];
    for &d in &digits {
        freq[d as usize] += 1;
    }
    let mut ans = 0;
    for hundreds in 1usize..=9 {
        for tens in 0usize..=9 {
            for ones in (0usize..=8).step_by(2) {
                let mut need = [0; 10];
                need[hundreds] += 1;
                need[tens] += 1;
                need[ones] += 1;

                if (0..10).all(|i| need[i] <= freq[i]) {
                    ans += 1;
                }
            }
        }
    }
    ans
}
