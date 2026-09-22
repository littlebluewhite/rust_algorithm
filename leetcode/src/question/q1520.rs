pub fn max_num_of_substrings(s: String) -> Vec<String> {
    let b = s.as_bytes();
    let n = b.len();
    let mut first = [n; 26];
    let mut last = [0usize; 26];
    for i in 0..n {
        let idx = (b[i] - b'a') as usize;
        if first[idx] == n {
            first[idx] = i;
        }
        last[idx] = i;
    }
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for start in 0..n {
        let idx = (b[start] - b'a') as usize;
        if first[idx] != start {
            continue;
        }
        let Some(end) = closure(b, &first, &last, start) else {
            continue;
        };
        if ranges.last().map_or(true, |&(_, r)| r < start) {
            ranges.push((start, end));
        } else {
            let last_range = ranges.len() - 1;
            ranges[last_range] = (start, end);
        }
    }
    ranges
        .into_iter()
        .map(|(start, end)| s[start..=end].to_string())
        .collect()
}

fn closure(b: &[u8], first: &[usize; 26], last: &[usize; 26], start: usize) -> Option<usize> {
    let mut end = last[(b[start]-b'a') as usize];
    let mut i = start;
    while i <= end {
        let idx = (b[i] - b'a') as usize;
        if first[idx] < start {
            return None;
        }
        end = end.max(last[idx]);
        i += 1;
    }
    Some(end)
}
