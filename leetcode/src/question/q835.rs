use std::collections::HashMap;

pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    let n = img1.len();
    let mut ones1: Vec<(i32, i32)> = Vec::new();
    let mut ones2: Vec<(i32, i32)> = Vec::new();
    for r in 0..n{
        for c in 0..n{
            if img1[r][c] == 1{
                ones1.push((r as i32, c as i32));
            }
            if img2[r][c] == 1{
                ones2.push((r as i32, c as i32));
            }
        }
    }
    let mut best = 0;
    let mut shift: HashMap<(i32, i32), i32> = HashMap::new();
    for &(r1, c1) in ones1.iter(){
        for &(r2, c2) in ones2.iter(){
            let m = (r1-r2, c1-c2);
            let count = shift.entry(m).or_insert(0);
            *count += 1;
            best = best.max(*count);
        }
    }
    best
}