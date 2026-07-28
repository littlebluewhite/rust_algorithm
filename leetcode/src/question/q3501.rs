struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(full_gain: &[i32]) -> Self {
        let n = full_gain.len();
        let mut tree = vec![0; 2 * n];
        tree[n..].copy_from_slice(full_gain);
        for i in (0..n).rev() {
            tree[i] = tree[2 * i].max(tree[2 * i + 1]);
        }
        Self { n, tree }
    }
    fn range_max(&self, l: usize, r: usize) -> i32 {
        let mut max = 0;
        let mut left = self.n + l;
        let mut right = self.n + r;
        while left <= right {
            if left & 1 == 1 {
                max = max.max(self.tree[left]);
                left += 1;
            }
            if right & 1 == 0 {
                max = max.max(self.tree[right]);
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        max
    }
}

#[derive(Clone, Copy)]
struct Candidate {
    left_start: usize,
    left_end: usize,
    right_start: usize,
    right_end: usize,
}

fn slice_candidate_gain(c: Candidate, l: usize, r: usize) -> i32 {
    let left = c.left_end - l.max(c.left_start) + 1;
    let right = r.min(c.right_end) - c.right_start + 1;
    (left + right) as i32
}

pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let b = s.as_bytes();
    let n = b.len();
    let all_ones = b.iter().filter(|&&c| c == b'1').count() as i32;
    let mut runs: Vec<(usize, usize, u8)> = Vec::new();
    let mut i = 0;
    while i < n {
        let start = i;
        let ch = b[i];
        while i < n && b[i] == ch {
            i += 1;
        }
        runs.push((start, i - 1, ch));
    }
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut full_gain = Vec::new();
    for run in runs.windows(3) {
        let (l, m, r) = (run[0], run[1], run[2]);
        if m.2 == b'1' {
            candidates.push(Candidate {
                left_start: l.0,
                left_end: l.1,
                right_start: r.0,
                right_end: r.1,
            });
            full_gain.push((l.1 - l.0 + 1 + r.1 - r.0 + 1) as i32);
        }
    }
    let segment_tree = SegmentTree::new(&full_gain);
    let mut ans = Vec::with_capacity(queries.len());
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        let first = candidates.partition_point(|c| c.left_end < l);
        let last_excluded = candidates.partition_point(|c| c.right_start <= r);
        if first >= last_excluded {
            ans.push(all_ones);
            continue;
        }
        let mut best_add = slice_candidate_gain(candidates[first], l, r).max(slice_candidate_gain(
            candidates[last_excluded - 1],
            l,
            r,
        ));
        if first + 3 <= last_excluded {
            best_add = best_add.max(segment_tree.range_max(first + 1, last_excluded - 2));
        }
        ans.push(best_add + all_ones);
    }
    ans
}
