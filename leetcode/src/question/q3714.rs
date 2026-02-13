use std::collections::HashMap;

pub fn longest_balanced(s: String) -> i32 {
    fn states(a: i32, b: i32, c: i32) -> [(i32, i32); 7] {
        [
            (b, c),         // a
            (a, c),         // b
            (a, b),         //c
            (a - b, c),     // a, b
            (a - c, b),     // a, c
            (b - c, a),     // b, c
            (a - b, a - c), // a, b, c
        ]
    }

    let mut maps: Vec<HashMap<(i32, i32), usize>> = vec![HashMap::new(); 7];
    let mut ans = 0usize;
    let mut cnt = [0;3];
    for i in 0..7{
        maps[i].insert((0, 0), 0);
    }
    for (i, c) in s.as_bytes().iter().enumerate(){
        match c{
            b'a' => cnt[0] += 1,
            b'b' => cnt[1] += 1,
            b'c' => cnt[2] += 1,
            _ => unreachable!()
        }
        let idx = i+1;
        let s = states(cnt[0], cnt[1], cnt[2]);
        for t in 0..7{
            if let Some(&j) = maps[t].get(&s[t]){
                ans = ans.max(idx-j);
            }else{
                maps[t].insert(s[t], idx);
            }
        }
    }
    ans as i32
}
