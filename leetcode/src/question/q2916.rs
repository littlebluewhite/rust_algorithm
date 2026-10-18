const MOD: i64 = 1000000007;

struct Fenwick {
    bit: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Fenwick {
        Fenwick {
            bit: vec![0; n + 1],
        }
    }
    fn add(&mut self, mut index: usize, delta: i64) {
        while index < self.bit.len() {
            self.bit[index] += delta;
            index += index & index.wrapping_neg();
        }
    }
    fn sum(&self, mut index: usize) -> i64 {
        let mut ans = 0;
        while index > 0 {
            ans += self.bit[index];
            index &= index - 1;
        }
        ans
    }
}

struct FenwickTree {
    difference: Fenwick,
    weighted: Fenwick,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            difference: Fenwick::new(n),
            weighted: Fenwick::new(n),
        }
    }
    fn range_add(&mut self, left: usize, right: usize, delta: i64) {
        self.difference.add(left, delta);
        self.difference.add(right + 1, -delta);
        self.weighted.add(left, delta * (left as i64 - 1));
        self.weighted.add(right + 1, -delta * (right as i64));
    }

    fn prefix_sum(&self, right: usize) -> i64 {
        right as i64 * self.difference.sum(right) - self.weighted.sum(right)
    }
    fn range_sum(&self, left: usize, right: usize) -> i64 {
        self.prefix_sum(right) - self.prefix_sum(left - 1)
    }
}

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let max_value = *nums.iter().max().unwrap_or(&0) as usize;
    let mut counts = FenwickTree::new(n);
    let mut last = vec![0; max_value + 1];
    let mut ending = 0;
    let mut ans = 0i64;
    for (idx, &value) in nums.iter().enumerate() {
        let right = idx + 1;
        let left = last[value as usize] + 1;
        let old_sum = counts.range_sum(left, right);
        ending = (ending + old_sum * 2 + (right - left + 1) as i64) % MOD;
        ans = (ans + ending) % MOD;
        last[value as usize] = right;
        counts.range_add(left, right, 1);
    }
    ans as i32
}
