use std::collections::{HashSet, VecDeque};

pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = VecDeque::new();
    let mut ans = s.clone();

    visited.insert(s.clone());
    queue.push_back(s);
    while let Some(curr) = queue.pop_front() {
        if curr < ans {
            ans = curr.clone();
        }
        let mut chars: Vec<char> = curr.chars().collect();
        for i in (0..chars.len()).step_by(2){
            let mut digit = chars[i].to_digit(10).unwrap();
            digit = (digit + a as u32) % 10;
            chars[i] = std::char::from_digit(digit, 10).unwrap();
        }
        let added = chars.iter().collect::<String>();
        if visited.insert(added.clone()){
            queue.push_back(added);
        }

        let b = b as usize;
        let n = curr.len();
        let rotated = format!("{}{}", &curr[n - b..], &curr[..n-b]);
        if visited.insert(rotated.clone()){
            queue.push_back(rotated);
        }
    }
    ans
}