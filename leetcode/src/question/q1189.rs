use std::collections::HashMap;

pub fn max_number_of_balloons(text: String) -> i32 {
    let b = text.as_bytes();
    let mut map: HashMap<u8, i32> = HashMap::new();
    for &c in b{
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    (*map.get(&b'b').unwrap_or(&0))
        .min(*map.get(&b'a').unwrap_or(&0))
        .min(*map.get(&b'n').unwrap_or(&0))
        .min(*map.get(&b'o').unwrap_or(&0)/2)
        .min(*map.get(&b'l').unwrap_or(&0)/2)
}