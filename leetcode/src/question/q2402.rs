use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut meetings = meetings
        .into_iter()
        .map(|m| (m[0] as i64, m[1] as i64))
        .collect::<Vec<(i64, i64)>>();
    meetings.sort_by_key(|m| m.0);
    let mut available: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut count = vec![0usize; n];
    for i in 0..n {
        available.push(Reverse(i));
    }
    for (start, end) in meetings {
        while let Some(&Reverse((busy_end, room))) = busy.peek(){
            if busy_end<=start{
                available.push(Reverse(room));
                busy.pop();
            }else{
                break;
            }
        }
        if let Some(Reverse(room)) = available.pop(){
            count[room]+=1;
            busy.push(Reverse((end, room)));
        }else{
            if let Some(Reverse((busy_end, room))) = busy.pop() {
                let delta = busy_end-start;
                count[room]+=1;
                busy.push(Reverse((end+delta, room)));
            }
        }
    }
    let mut best = 0usize;
    for room in 1..n{
        if count[room]>count[best]{
            best = room
        }
    }
    best as i32
}
