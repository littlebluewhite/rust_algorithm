pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    const MOD: u64 = 1000000007;
    let m = (r-l+1) as usize;
    let mut up = vec![1u64; m];
    let mut down = vec![1u64; m];
    for _ in 2..=n{
        let mut acc: u64 = 0;
        let mut new_up = vec![0u64; m];
        let mut new_down = vec![0u64; m];
        for j in 0..m{
            new_up[j] = acc;
            acc = (acc +down[j])%MOD;
        }
        acc = 0;
        for j in (0..m).rev(){
            new_down[j] = acc;
            acc = (acc +up[j])%MOD;
        }
        up = new_up;
        down = new_down;
    }
    let mut ans = 0u64;
    for i in 0..m{
        ans = (ans + up[i] + down[i])%MOD;
    }
    ans as i32
}