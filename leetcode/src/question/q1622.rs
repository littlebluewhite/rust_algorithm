const MOD: i64 = 1_000_000_007;

struct Fancy {
    vals: Vec<i64>,
    mul: i64,
    add: i64,
}

impl Fancy {
    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut ans = 1_i64;
        base %= MOD;
        while exp > 0 {
            if exp & 1 == 1 {
                ans = ans * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }
        ans
    }

    fn inv(x: i64) -> i64 {
        Self::mod_pow(x, MOD - 2)
    }

    fn new() -> Self {
        Self {
            vals: Vec::new(),
            mul: 1,
            add: 0,
        }
    }

    fn append(&mut self, val: i32) {
        let normalized = ((val as i64 - self.add + MOD) % MOD) * Self::inv(self.mul) % MOD;
        self.vals.push(normalized);
    }

    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }

    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.mul = self.mul * m % MOD;
        self.add = self.add * m % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.vals.len() {
            return -1;
        }
        ((self.vals[idx] * self.mul + self.add) % MOD) as i32
    }
}
