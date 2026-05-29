pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    let len: Vec<usize> = words_container.iter().map(|s| s.len()).collect();
    let capacity: usize = len.iter().sum();
    let mut children: Vec<[i32; 26]> = Vec::with_capacity(capacity);
    let mut best: Vec<i32> = Vec::with_capacity(capacity);
    children.push([-1; 26]);
    best.push(0);
    for (i, word) in words_container.into_iter().enumerate() {
        let mut cur = 0usize;
        update_best(&mut best[cur], i, &len);
        for &b in word.as_bytes().iter().rev() {
            let c = (b - b'a') as usize;
            if children[cur][c] == -1 {
                children.push([-1; 26]);
                best.push(-1);
                children[cur][c] = (children.len() - 1) as i32;
            }
            cur = children[cur][c] as usize;
            update_best(&mut best[cur], i, &len);
        }
    }
    let mut ans = Vec::with_capacity(words_query.len());
    for i in words_query.into_iter() {
        let mut cur = 0usize;
        let mut next = best[cur];
        for &b in i.as_bytes().iter().rev() {
            let c = (b - b'a') as usize;
            if children[cur][c] == -1 {
                break;
            }
            cur = children[cur][c] as usize;
            next = best[cur];
        }
        ans.push(next);
    }
    ans
}

fn update_best(best: &mut i32, candidate: usize, len: &[usize]) {
    if *best == -1 || is_better(candidate, *best as usize, len) {
        *best = candidate as i32;
    }
}

fn is_better(candidate: usize, best: usize, len: &[usize]) -> bool {
    if len[candidate] < len[best] || (len[candidate] == len[best] && candidate < best) {
        return true;
    }
    false
}
