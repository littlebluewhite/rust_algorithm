use std::collections::BinaryHeap;

struct SparseTable{
    log2: Vec<usize>,
    mx: Vec<Vec<i32>>,
    mn: Vec<Vec<i32>>,
}

impl SparseTable{
    fn new(nums: Vec<i32>) -> Self{
        let n = nums.len();
        let mut log2 = vec![0; n+1];
        for i in 2..=n {
            log2[i] = log2[i/2] + 1;
        }
        let levels = log2[n]+1;
        let mut mx = vec![vec![0; n]; levels];
        let mut mn = vec![vec![0; n]; levels];
        mx[0].copy_from_slice(&nums);
        mn[0].copy_from_slice(&nums);
        for level in 1..levels{
            let len = 1 << level;
            let half = len >> 1;
            for i in 0..=(n - len){
                mx[level][i] = mx[level-1][i].max(mx[level-1][i+half]);
                mn[level][i] = mn[level-1][i].min(mn[level-1][i+half]);
            }
        }
        Self{log2, mx, mn}
    }

    fn range_max(&self, l: usize, r: usize) -> i32{
        let level = self.log2[r-l+1];
        let width = 1 << level;
        self.mx[level][l].max(self.mx[level][r-width+1])
    }

    fn range_min(&self, l: usize, r: usize) -> i32{
        let level = self.log2[r-l+1];
        let width = 1 << level;
        self.mn[level][l].min(self.mn[level][r-width+1])
    }

    fn value(&self, l: usize, r: usize) -> i64{
        self.range_max(l, r) as i64- self.range_min(l, r) as i64
    }
}

pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let sparse_table = SparseTable::new(nums);
    let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    for l in 0..n-1{
        heap.push((sparse_table.value(l, n-1), l, n-1));
    }
    let mut ans = 0i64;
    for _ in 0..k{
        if let Some((val, l, r)) = heap.pop(){
            ans += val;
            if r > l+1{
                heap.push((sparse_table.value(l, r-1), l, r-1));
            }
        }
    }
    ans
}