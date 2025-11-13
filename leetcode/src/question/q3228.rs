pub fn max_operations(s: String) -> i32 {
    let mut ans = 0i32;
    let mut ones = 0i32;
    let mut pre = b'0';
    for b in s.bytes(){
        if b == b'1'{
            ones += 1;
        }else{
            if pre == b'1'{
                ans += ones;
            }
        }
        pre = b;
    }
    ans
}