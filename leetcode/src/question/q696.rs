pub fn count_binary_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let mut pre = 0;
    let mut cur = 1;
    let mut ans = 0;
    for i in 1.. s.len(){
        if s[i] == s[i-1]{
            cur += 1;
        }else{
            ans += pre.min(cur);
            pre = cur;
            cur = 1;
        }
    }
    ans + pre.min(cur)
}