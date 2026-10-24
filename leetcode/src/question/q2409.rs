const MONTH: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

pub fn count_days_together(
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
) -> i32 {
    let a_start = to_day(&arrive_alice);
    let a_end = to_day(&leave_alice);
    let b_start = to_day(&arrive_bob);
    let b_end = to_day(&leave_bob);
    (a_end.min(b_end) - a_start.max(b_start) + 1).max(0)
}

fn to_day(date: &str) -> i32{
    let b = date.as_bytes();
    let month = (b[0]-b'0') as usize *10+(b[1]-b'0')as usize;
    let day = (b[3]-b'0') as i32 *10+(b[4]-b'0')as i32;
    MONTH[month-1] + day
}