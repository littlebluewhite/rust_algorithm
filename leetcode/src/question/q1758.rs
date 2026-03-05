
pub fn min_operations(s: String) -> i32 {
    let s = s.as_bytes();
    let mut mismatch_star0 = 0;
    let mut mismatch_star1 = 0;
    for (index, &ch) in s.iter().enumerate(){
        let expect_star0 = if index%2==0{b'0'}else{b'1'};
        let expect_star1 = if index%2==0{b'1'}else{b'0'};
        if expect_star0 != ch{
            mismatch_star0+=1;
        }
        if expect_star1 != ch{
            mismatch_star1+=1;
        }
    }
    mismatch_star1.min(mismatch_star0)
}

pub fn min_operations2(s: String) -> i32 {
    let mut res = 0;
    let mut bit = 0u8;
    for c in s.bytes() {
        res += ((c - b'0') ^ bit) as i32;
        bit ^= 1;
    }
    return res.min(s.len() as i32 - res);
}