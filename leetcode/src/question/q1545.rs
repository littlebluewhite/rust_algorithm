pub fn find_kth_bit(n: i32, k: i32) -> char {
    fn apply_flip(bit: char, flip: bool) -> char {
        if !flip {
            bit
        } else {
            if bit == '0' { '1' } else { '0' }
        }
    }

    fn dfs(n: i32, k: i32, flip: bool) -> char {
        if n == 1 {
            return apply_flip('0', flip);
        }

        let mid = 1 << (n - 1);
        if k == mid {
            return apply_flip('1', flip);
        }

        if k < mid {
            dfs(n - 1, k, flip)
        } else {
            dfs(n - 1, (1 << n) - k, !flip)
        }
    }
    dfs(n, k, false)
}
