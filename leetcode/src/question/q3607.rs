use std::collections::{BTreeSet, HashMap};

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.parent[x];
            self.parent[x] = self.find(root);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            self.parent[root_b] = root_a;
        }
    }
}

pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut dsu = DSU::new(c as usize);
    for conn in connections {
        dsu.union(conn[0] as usize, conn[1] as usize);
    }
    let mut group: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    let mut tidy_roots = vec![0; c as usize + 1];

    for i in 1..=c as usize {
        let root = dsu.find(i);
        tidy_roots[i] = root;
        group.entry(root).or_default().insert(i);
    }

    let mut res: Vec<i32> = Vec::new();
    for query in queries {
        let x = query[1] as usize;
        let root = tidy_roots[x];
        if query[0] == 1 {
            if let Some(set) = group.get_mut(&root) {
                if set.contains(&x) {
                    res.push(x as i32);
                } else if let Some(&min_x) = set.iter().next() {
                    res.push(min_x as i32);
                } else {
                    res.push(-1);
                }
            } else {
                res.push(-1)
            }
        } else {
            if let Some(set) = group.get_mut(&root) {
                set.remove(&x);
            }
        }
    }
    res
}
