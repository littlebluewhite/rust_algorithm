pub fn longest_balanced(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut ans = 0;
    for i in 0..n{
        let mut seen = vec![0;26];
        let mut max_freq = 0;
        let mut distinct_count = 0;
        for j in i..n{
            let c = (s[j]  - b'a') as usize;
            seen[c] += 1;
            if seen[c] == 1{
                distinct_count += 1;
            }
            max_freq = max_freq.max(seen[c]);
            let len = j-i+1;
            if len == distinct_count* max_freq {
                ans = ans.max(len);
            }
        }
    }
    ans as i32
}