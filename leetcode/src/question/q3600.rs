#[derive(Clone)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if a == self.parent[a] {
            return a;
        }
        self.parent[a] = self.find(self.parent[a]);
        self.parent[a]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    strength: i32,
}

fn can_build(
    target: i32,
    n: usize,
    k: i32,
    must_used: usize,
    min_strength: i32,
    dsu: &Dsu,
    optional: &[Edge],
) -> bool {
    if min_strength != i32::MAX && target > min_strength {
        return false;
    }
    let mut dsu = dsu.clone();
    let mut upgraded_time = k;
    let mut used = must_used;
    if used == n - 1 {
        return true;
    }
    for edge in optional {
        if used == n - 1 {
            return true;
        }
        let ur = dsu.find(edge.u);
        let vr = dsu.find(edge.v);
        if ur == vr {
            continue;
        }
        if edge.strength >= target {
            dsu.union(ur, vr);
            used += 1;
            continue;
        }

        if upgraded_time > 0 && edge.strength * 2 >= target {
            dsu.union(ur, vr);
            upgraded_time -= 1;
            used += 1;
            continue;
        }
        break;
    }
    used == n - 1
}

pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = n as usize;
    let mut dsu = Dsu::new(n);
    let mut max_strength = 0i32;
    let mut mandatory: Vec<Edge> = Vec::new();
    let mut optional: Vec<Edge> = Vec::new();
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let strength = edge[2];
        let must = edge[3];
        max_strength = max_strength.max(strength * 2);
        if must == 1 {
            mandatory.push(Edge { u, v, strength });
        } else {
            optional.push(Edge { u, v, strength });
        }
    }

    optional.sort_unstable_by(|a, b| b.strength.cmp(&a.strength));

    let mut min_mandatory_strength = i32::MAX;
    for edge in mandatory.iter() {
        min_mandatory_strength = min_mandatory_strength.min(edge.strength);
        if !dsu.union(edge.u, edge.v) {
            return -1;
        }
    }
    if !can_build(
        0,
        n,
        k,
        mandatory.len(),
        min_mandatory_strength,
        &dsu,
        &optional,
    ) {
        return -1;
    }
    let mut hi = max_strength * 2;
    if min_mandatory_strength != i32::MAX {
        hi = hi.min(min_mandatory_strength);
    }
    let mut lo = 0;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if can_build(
            mid,
            n,
            k,
            mandatory.len(),
            min_mandatory_strength,
            &dsu,
            &optional,
        ) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}
