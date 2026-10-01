use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    let mut stack: Vec<u8> = Vec::with_capacity(n);
    let mut map: HashMap<u8, u8> = HashMap::new();
    map.insert(b')', b'(');
    map.insert(b']', b'[');
    map.insert(b'}', b'{');
    for i in 0..n{
        if map.contains_key(&b[i]){
            if stack.is_empty() || stack.pop() != Some(map[&b[i]]){
                return false;
            }
        }else{
            stack.push(b[i]);
        }
    }
    stack.is_empty()
}