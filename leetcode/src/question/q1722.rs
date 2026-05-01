use std::collections::HashMap;

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

pub fn minimum_hamming_distance(
    source: Vec<i32>,
    target: Vec<i32>,
    allowed_swaps: Vec<Vec<i32>>,
) -> i32 {
    let n = source.len();
    let mut dsu = DSU::new(n);
    for edge in allowed_swaps {
        dsu.union(edge[0] as usize, edge[1] as usize);
    }
    let mut group: HashMap<usize, HashMap<i32, i32>> = HashMap::new();
    for i in 0..n {
        let root = dsu.find(i);
        *group
            .entry(root)
            .or_insert(HashMap::new())
            .entry(source[i])
            .or_insert(0) += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        let root = dsu.find(i);
        let counts = group.get_mut(&root).unwrap();
        if let Some(count) = counts.get_mut(&target[i]) {
            if *count > 0 {
                *count -= 1;
                continue;
            }
        }
        ans += 1;
    }
    ans
}
