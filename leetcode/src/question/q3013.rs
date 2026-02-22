use std::collections::BTreeMap;

struct MultiSet {
    size: usize,
    mp: BTreeMap<i32, i32>,
}

impl MultiSet {
    fn new() -> Self {
        Self {
            size: 0,
            mp: BTreeMap::new(),
        }
    }

    fn insert(&mut self, x: i32) {
        *self.mp.entry(x).or_insert(0) += 1;
        self.size += 1;
    }

    fn remove(&mut self, x: i32) -> bool {
        if let Some(cnt) = self.mp.get_mut(&x) {
            *cnt -= 1;
            if *cnt == 0 {
                self.mp.remove(&x);
            }
            self.size -= 1;
            true
        } else {
            false
        }
    }
    fn min_key(&self) -> i32 {
        *self.mp.first_key_value().unwrap().0
    }
    fn max_key(&self) -> i32 {
        *self.mp.last_key_value().unwrap().0
    }
    fn pop_min(&mut self) -> i32 {
        let x = self.min_key();
        self.remove(x);
        x
    }
    fn pop_max(&mut self) -> i32 {
        let x = self.max_key();
        self.remove(x);
        x
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

struct Container {
    m: usize,
    st1: MultiSet,
    st2: MultiSet,
    sum1: i64,
}

impl Container {
    fn new(m: usize) -> Self {
        Self {
            m,
            st1: MultiSet::new(),
            st2: MultiSet::new(),
            sum1: 0,
        }
    }
    fn rebalance(&mut self) {
        while self.st1.size > self.m {
            let x = self.st1.pop_max();
            self.st2.insert(x);
            self.sum1 -= x as i64;
        }
        while self.st1.size < self.m && !self.st2.is_empty() {
            let x = self.st2.pop_min();
            self.st1.insert(x);
            self.sum1 += x as i64;
        }
    }

    fn add(&mut self, x: i32) {
        if !self.st2.is_empty() {
            if self.st2.min_key() <= x {
                self.st2.insert(x)
            } else {
                self.st1.insert(x);
                self.sum1 += x as i64;
            }
        } else {
            self.st1.insert(x);
            self.sum1 += x as i64;
        }
        self.rebalance();
    }

    fn erase(&mut self, x: i32) {
        if self.st1.remove(x) {
            self.sum1 -= x as i64;
        } else {
            self.st2.remove(x);
        }
        self.rebalance();
    }
}

pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
    let n = nums.len();
    let m = (k as usize).saturating_sub(2);
    let mut container = Container::new(m);
    for i in 1..(k as usize) - 1 {
        container.add(nums[i]);
    }

    let mut ans = container.sum1 + nums[k as usize-1] as i64;
    for i in (k as usize)..n{
        let j = i as i32 -dist-1;
        if j > 0{
            container.erase(nums[j as usize]);
        }
        container.add(nums[i-1]);
        ans = ans.min(container.sum1 + nums[i] as i64);

    }
    ans + nums[0] as i64
}
