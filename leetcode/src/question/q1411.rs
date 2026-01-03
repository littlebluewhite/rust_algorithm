const MOD: i64 = 1_000_000_007;
pub fn num_of_ways(n: i32) -> i32 {
    if n == 1 {
        return 12;
    }
    let mut a = 6i64; //ABC
    let mut b = 6i64; //ABA
    for _ in 1..n{
        let new_a = (a*2+b*2)%MOD;
        let new_b = (a*2+b*3)%MOD;
        a = new_a;
        b = new_b;
    }
    ((a+b)%MOD) as i32
}
