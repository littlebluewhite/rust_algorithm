use std::collections::HashMap;

pub fn number_of_substrings(s: String) -> i32 {
    let mut ans = 0;
    let b = s.as_bytes();
    let mut dp: HashMap<(usize, usize), (i32, i32)> = HashMap::new();
    let mut zeros = 0;
    let mut ones = 0;
    for i in 0..b.len(){
        if b[i] == b'0'{
            zeros += 1;
        }else{
            ones += 1;
        }
        if ones >= zeros*zeros{
            ans += 1;
        }
        dp.insert((0, i), (zeros, ones));
    }
    for i in 1..b.len() {
        for j in i..b.len() {
            let &(mut zeros, mut ones) = dp.get(&(i - 1, j)).unwrap();
            if b[i - 1] == b'0' {
                zeros -= 1;
            } else {
                ones -= 1;
            }
            dp.insert((i, j), (zeros, ones));
            if ones >= zeros * zeros {
                ans += 1;
            }
        }
    }
    ans
}