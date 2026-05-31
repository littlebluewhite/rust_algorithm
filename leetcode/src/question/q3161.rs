use std::collections::BTreeSet;

struct SegTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegTree {
    fn init(size: usize) -> Self {
        let mut n = 1usize;
        while n < size {
            n <<= 1;
        }
        Self {
            n,
            tree: vec![0; n * 2],
        }
    }

    fn update(&mut self, index: usize, value: i32) {
        let mut p = self.n + index;
        self.tree[p] = value;
        p >>= 1;

        while p > 0 {
            self.tree[p] = self.tree[p << 1].max(self.tree[p << 1 | 1]);
            p >>= 1;
        }
    }

    fn range_max(&self, left: usize, right: usize) -> i32 {
        if left > right {
            return 0;
        }
        let mut l = self.n + left;
        let mut r = self.n + right + 1;
        let mut ans = 0;
        while l < r {
            while l & 1 == 1 {
                ans = self.tree[l].max(ans);
                l += 1
            }
            while r & 1 == 1 {
                r -= 1;
                ans = self.tree[r].max(ans);
            }
            l >>= 1;
            r >>= 1;
        }
        ans
    }
}

pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
    let max_count = queries
        .iter()
        .flat_map(|q| q.iter().skip(1))
        .copied()
        .max()
        .unwrap_or(0);
    let mut seg = SegTree::init(max_count as usize + 1);
    let mut obstacle: BTreeSet<i32> = BTreeSet::new();
    obstacle.insert(0);
    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let x = q[1];
            let pre = obstacle.range(0..=x).next_back().copied().unwrap();
            let next = obstacle.range((x + 1)..).next().copied();

            seg.update(x as usize, x - pre);
            if let Some(next) = next {
                seg.update(next as usize, next - x);
            }

            obstacle.insert(x);
        } else {
            let x = q[1];
            let sz = q[2];
            let pre = obstacle.range(0..=x).next_back().copied().unwrap();

            let best_complete = seg.range_max(0, pre as usize);
            let best_partial = x - pre;
            ans.push(best_complete.max(best_partial) >= sz);
        }
    }
    ans
}
