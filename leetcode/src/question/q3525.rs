pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let mut tree = SegTree::new(&nums, k as usize);
    let n = nums.len();
    queries
        .iter()
        .map(|query| {
            tree.set(query[0] as usize, query[1]);
            tree.query(query[2] as usize, n).cnt[query[3] as usize]
        })
        .collect()
}

const MAX_K: usize = 5;

#[derive(Clone, Copy)]
struct Seg {
    prod: usize,
    cnt: [i64; MAX_K],
}

impl Seg {
    fn identify(k: usize) -> Self {
        Self {
            prod: 1,
            cnt: [0; MAX_K],
        }
    }
    fn leaf(value: i32, k: usize) -> Self {
        let residue = value as usize % k;
        let mut cnt = [0; MAX_K];
        cnt[residue] += 1;
        Self { prod: residue, cnt }
    }
}

fn concat(left: Seg, right: Seg, k: usize) -> Seg {
    let mut cnt = left.cnt;
    for i in 0..k {
        cnt[(left.prod * i) % k] += right.cnt[i];
    }
    Seg {
        prod: (left.prod * right.prod) % k,
        cnt,
    }
}

struct SegTree {
    size: usize,
    k: usize,
    tree: Vec<Seg>,
}

impl SegTree {
    fn new(nums: &[i32], k: usize) -> Self {
        let size = nums.len().next_power_of_two();
        let mut tree = vec![Seg::identify(k); size * 2];
        for (i, &value) in nums.iter().enumerate() {
            tree[size + i] = Seg::leaf(value, k);
        }
        for i in (1..size).rev() {
            tree[i] = concat(tree[i * 2], tree[i * 2 + 1], k);
        }
        Self { size, k, tree }
    }

    fn set(&mut self, index: usize, value: i32) {
        let mut pos = self.size + index;
        self.tree[pos] = Seg::leaf(value, self.k);
        while pos > 1 {
            pos /= 2;
            self.tree[pos] = concat(self.tree[pos * 2], self.tree[pos * 2 + 1], self.k);
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> Seg {
        left = left + self.size;
        right = right + self.size;
        let mut head = Seg::identify(self.k);
        let mut tail = Seg::identify(self.k);

        while left < right {
            if left & 1 == 1 {
                head = concat(head, self.tree[left], self.k);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                tail = concat(self.tree[right], tail, self.k);
            }
            left /= 2;
            right /= 2;
        }
        concat(head, tail, self.k)
    }
}
