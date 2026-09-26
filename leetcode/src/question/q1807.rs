use std::collections::HashMap;

pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut ans = String::with_capacity(s.len());
    let mut map: HashMap<&str, &str> = knowledge
        .iter()
        .map(|v| (v[0].as_str(), v[1].as_str()))
        .collect();
    let mut i = 0;
    while i < n {
        if b[i] == b'(' {
            let start = i + 1;
            i = start;
            while b[i] != b')' {
                i += 1;
            }
            let key = &s[start..i];
            ans.push_str(map.get(key).copied().unwrap_or("?"));
            i += 1;
        } else {
            ans.push(b[i] as char);
            i += 1;
        }
    }
    ans
}
