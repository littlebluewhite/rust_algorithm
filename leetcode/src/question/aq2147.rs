const MOD:i64 = 1_000_000_007;

pub fn number_of_ways(corridor: String) -> i32 {
    let mut count_seat = 0i32;
    let mut seat_to_set: Vec<(usize, usize)> = Vec::new();
    let mut pre = 0usize;
    for i in 0..corridor.len() {
        if corridor.chars().nth(i).unwrap() == 'S' {
            count_seat += 1;
            if count_seat >= 2 && count_seat % 2 == 0 {
                pre = i;
            }
            if count_seat >= 2 && count_seat % 2 == 1 && pre != i-1{
                seat_to_set.push((pre, i));
            }
        }
    }
    if count_seat % 2 != 0 || count_seat == 0 {
        return 0;
    }
    let mut ans = 1i64;
    for (l, r) in seat_to_set {
        println!("{} {}", l, r);
        ans = (ans * (r - l) as i64) % MOD;
    }
    ans as i32
}