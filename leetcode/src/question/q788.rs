pub fn rotated_digits(n: i32) -> i32 {
    let mut count = 0;
    'a: for i in 1..=n {
        let mut good = false;
        let mut current = i;
        while current > 0 {
            let o = current % 10;
            match o {
                3 | 4 | 7 => {
                    continue 'a;
                }
                2 | 5 | 6 | 9 => {
                    good = true;
                }
                _ => {}
            }
            current /= 10;
        }
        if good {
            count += 1;
        }
    }
    count
}
