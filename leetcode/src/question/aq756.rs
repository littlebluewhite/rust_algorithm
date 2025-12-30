use std::collections::HashMap;

pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    let mut trans: Vec<Vec<Vec<u8>>> = vec![vec![Vec::new(); 6]; 6];
    for s in allowed {
        let b = s.as_bytes();
        let a = (b[0] - b'A') as usize;
        let c = (b[1] - b'A') as usize;
        trans[a][c].push(b[2]);
    }

    let mut memo: HashMap<Vec<u8>, bool> = HashMap::new();
    let row = bottom.into_bytes();
    can_build(row, &trans, &mut memo)
}

fn can_build(row: Vec<u8>, trans: &Vec<Vec<Vec<u8>>>, memo: &mut HashMap<Vec<u8>, bool>) -> bool {
    if row.len() == 1 {
        return true;
    }
    if let Some(&cached) = memo.get(&row) {
        return cached;
    }

    let mut next: Vec<u8> = Vec::with_capacity(row.len() - 1);
    let ok = build_next(0, &row, &mut next, trans, memo);
    memo.insert(row, ok);
    ok
}

fn build_next(
    idx: usize,
    row: &[u8],
    next: &mut Vec<u8>,
    trans: &Vec<Vec<Vec<u8>>>,
    memo: &mut HashMap<Vec<u8>, bool>,
) -> bool {
    if idx == row.len() - 1 {
        return can_build(next.clone(), trans, memo);
    }
    let a = (row[idx] - b'A') as usize;
    let b = (row[idx + 1] - b'A') as usize;
    for &top in trans[a][b].iter() {
        next.push(top);
        if build_next(idx + 1, row, next, trans, memo) {
            return true;
        }
        next.pop();
    }
    false
}
