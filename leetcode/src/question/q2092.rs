use std::collections::HashSet;

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }
    fn union(&mut self, x: usize, y: usize) {
        let mut rx = self.find(x);
        let mut ry = self.find(y);
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        self.parent[ry] = rx;
        self.size[rx] += self.size[ry];
    }
    fn reset(&mut self, x: usize) {
        self.parent[x] = x;
        self.size[x] = 1;
    }
}

pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    meetings.sort_by_key(|x| x[2]);
    let mut dsu = DSU::new(n as usize);
    dsu.union(0, first_person as usize);
    let mut i = 0;
    while i < meetings.len(){
        let time = meetings[i][2];
        let mut batch: Vec<(i32, i32)> = Vec::new();
        let mut involved: HashSet<usize> = HashSet::new();
        while i < meetings.len() && meetings[i][2] == time{
            batch.push((meetings[i][0], meetings[i][1]));
            involved.insert(meetings[i][0] as usize);
            involved.insert(meetings[i][1] as usize);
            i += 1;
        }
        for (a, b) in batch{
            dsu.union(a as usize, b as usize);
        }
        let root0 = dsu.find(0);
        for p in involved{
            if dsu.find(p) != root0{
                dsu.reset(p);
            }
        }
    }
    let root0 = dsu.find(0);
    let mut ans: Vec<i32> = Vec::new();
    for person in 0..n{
        if dsu.find(person as usize) == root0{
            ans.push(person as i32);
        }
    }
    ans
}
