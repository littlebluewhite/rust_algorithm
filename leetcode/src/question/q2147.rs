const MOD:i64 = 1_000_000_007;

pub fn number_of_ways(corridor: String) -> i32 {
    // LeetCode 2147: Number of Ways to Divide a Long Corridor
    // Key: scan once. Each partition must contain exactly 2 seats.
    // For every gap between seat-pairs, multiply by (plants_between_pairs + 1).

    let bytes = corridor.as_bytes();
    let mut seat_cnt: i32 = 0;

    // After finishing a pair (i.e., seat_cnt becomes even), we start counting plants
    // until we see the next seat (which starts the next pair). That plant count
    // determines the number of cut positions.
    let mut plants_between_pairs: i64 = 0;
    let mut ans: i64 = 1;

    for &b in bytes {
        if b == b'S' {
            // If we are about to start a new pair and we already have at least one full pair,
            // the number of choices for placing a divider between the previous pair and this seat
            // is (plants_between_pairs + 1).
            if seat_cnt > 0 && seat_cnt % 2 == 0 {
                ans = (ans * (plants_between_pairs + 1)) % MOD;
                plants_between_pairs = 0;
            }
            seat_cnt += 1;
        } else if b == b'P' {
            // Count plants only after completing a pair.
            if seat_cnt > 0 && seat_cnt % 2 == 0 {
                plants_between_pairs += 1;
            }
        }
    }

    if seat_cnt == 0 || seat_cnt % 2 != 0 {
        return 0;
    }

    ans as i32
}