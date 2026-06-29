struct Fenwick{
    vec: Vec<i64>
}

impl Fenwick{
    fn new(size: usize) -> Self{
        Self{
            vec: vec![0; size]
        }
    }
    fn update(&mut self, mut index: usize, value: i64){
        while  index< self.vec.len(){
            self.vec[index] += value;
            index += index & index.wrapping_neg();
        }
    }
    fn query(&self, mut index: usize) -> i64{
        let mut s = 0i64;
        while index > 0 {
            s += self.vec[index];
            index -= index & index.wrapping_neg();
        }
        s
    }
}

pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
    let n = nums.len();
    let size = 2 * n +1;
    let mut fenwick = Fenwick::new(size + 1);
    let to_idx = |v: i64| -> usize {n+v as usize+1};
    fenwick.update(to_idx(0), 1);
    let mut ans = 0i64;
    let mut cur = 0i64;
    for &num in &nums{
        cur += if num==target {1} else {-1};
        ans += fenwick.query(to_idx(cur-1));
        fenwick.update(to_idx(cur), 1);
    }
    ans
}

pub fn count_majority_subarrays2(nums: Vec<i32>, target: i32) -> i64 {
    let n = nums.len();
    let size = 2 * n +1;
    let to_idx = |v: i64| -> usize {n+v as usize+1};
    let mut cnt = vec![0i64; size+1];
    let mut cur = 0i64;
    let mut ans = 0i64;
    let mut less = 0i64;
    cnt[to_idx(0)] = 1;
    for &num in &nums{
        if num==target{
            cur += 1;
            less += cnt[to_idx(cur-1)];
        }else{
            cur -= 1;
            less -= cnt[to_idx(cur)];
        }
        cnt[to_idx(cur)] += 1;
        ans += less;
    }
    ans
}