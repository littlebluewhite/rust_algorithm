pub fn longest_balanced(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut ans = 0;
    for i in 0..n{
        let mut seen = vec![0;26];
        'a: for j in i..n{
            let c = (s[j]  - b'a') as usize;
            seen[c] += 1;
            for k in 0..26{
                if seen[k] > 0 && seen[k] != seen[c] {
                    continue 'a;
                }
            }
            ans = ans.max(j-i+1);
        }
    }
    ans as i32
}