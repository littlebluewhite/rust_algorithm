pub fn num_sub(s: String) -> i32 {
    let r#mod:i64 = 1_000_000_007;
    let mut count = 0;
    let mut ans = 0;

    for &b in s.as_bytes(){
        if b == b'1'{
            count += 1;
        }else{
            ans += ((1+count)*count/2)% r#mod;
            count = 0;
        }
    }
    if count >0{
        ans += ((1+count)*count/2)% r#mod;
    }
    ans as i32
}
