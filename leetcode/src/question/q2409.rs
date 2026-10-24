pub fn count_days_together(
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
) -> i32 {
    let month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let bytes1 = arrive_alice.as_bytes();
    let bytes2 = leave_alice.as_bytes();
    let bytes3 = arrive_bob.as_bytes();
    let bytes4 = leave_bob.as_bytes();
    let a_start_month = ((bytes1[0]-b'0')*10+(bytes1[1]-b'0')) as i32;
    let a_start_day = ((bytes1[3]-b'0')*10+(bytes1[4]-b'0'))as i32;

    let a_end_month = ((bytes2[0]-b'0')*10+(bytes2[1]-b'0'))as i32;
    let a_end_day = ((bytes2[3]-b'0')*10+(bytes2[4]-b'0'))as i32;
    let b_start_month = ((bytes3[0]-b'0')*10+(bytes3[1]-b'0'))as i32;
    let b_start_day = ((bytes3[3]-b'0')*10+(bytes3[4]-b'0'))as i32;
    let b_end_month = ((bytes4[0]-b'0')*10+(bytes4[1]-b'0'))as i32;
    let b_end_day = ((bytes4[3]-b'0')*10+(bytes4[4]-b'0'))as i32;
    let mut a_start = 0;
    let mut a_end = 0;
    let mut b_start = 0;
    let mut b_end = 0;
    for i in 1..13{
        if a_start_month > i{
            a_start += month[i as usize-1];
        }
        if a_end_month > i{
            a_end += month[i as usize-1];
        }
        if b_start_month > i{
            b_start += month[i as usize-1];
        }
        if b_end_month > i{
            b_end += month[i as usize-1];
        }
    }
    a_start += a_start_day;
    a_end += a_end_day;
    b_start += b_start_day;
    b_end += b_end_day;
    (a_end.min(b_end)-a_start.max(b_start)+1).max(0)
}
