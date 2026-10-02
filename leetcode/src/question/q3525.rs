pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = nums.len();
    let k = k as usize;
    let mut seg_tree = SegTree::new(&nums, k);
    queries
        .iter()
        .map(|query| {
            let (set_idx, set_value, start, r) = (
                query[0] as usize,
                query[1],
                query[2] as usize,
                query[3] as usize,
            );
            seg_tree.set(set_idx, set_value);
            seg_tree.query(start, n).cnt[r]
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
    fn identify() -> Self {
        Self {
            prod: 1,
            cnt: [0; MAX_K],
        }
    }
    fn leaf(value: i32, k: usize) -> Self {
        let prod = value as usize % k;
        let mut cnt = [0; MAX_K];
        cnt[prod] += 1;
        Self { prod, cnt }
    }
}

fn concat(left: Seg, right: Seg, k: usize) -> Seg {
    let mut cnt = left.cnt;
    for r in 0..k {
        cnt[(left.prod * r) % k] += right.cnt[r];
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
        let mut tree = vec![Seg::identify(); size * 2];
        for (i, &value) in nums.iter().enumerate() {
            tree[size + i] = Seg::leaf(value, k);
        }
        for i in (1..size).rev() {
            tree[i] = concat(tree[i * 2], tree[i * 2 + 1], k);
        }
        Self { size, k, tree }
    }

    fn set(&mut self, idx: usize, value: i32) {
        let mut index = idx + self.size;
        self.tree[index] = Seg::leaf(value, self.k);
        while index > 1 {
            index /= 2;
            self.tree[index] = concat(self.tree[index * 2], self.tree[index * 2 + 1], self.k);
        }
    }

    fn query(&self, mut left: usize, mut right: usize) -> Seg {
        let mut head = Seg::identify();
        let mut tail = Seg::identify();
        left += self.size;
        right += self.size;
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
