pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut even = 1i64;
    let mut odd = 0i64;
    let mut now = 0i64;

    for &num in &arr {
        let num = num as i64;
        now ^= num;
        if now &1== 0{
            even+=1;
        }else{
            odd+=1;
        }
    }
    ((even%MOD)*(odd%MOD)%MOD) as i32
}