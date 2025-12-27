use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut meetings: Vec<(i64, i64)> = meetings
        .into_iter()
        .map(|m| (m[0] as i64, m[1] as i64))
        .collect();
    meetings.sort_by_key(|m| m.0);

    let mut available: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for room in 0..n {
        available.push(Reverse(room));
    }
    let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut count = vec![0usize; n];

    for (start, end) in meetings {
        while let Some(&Reverse((t, room))) = busy.peek() {
            if t <= start {
                busy.pop();
                available.push(Reverse(room));
            } else {
                break;
            }
        }

        if let Some(Reverse(room)) = available.pop() {
            count[room] += 1;
            busy.push(Reverse((end, room)));
        } else {
            let Reverse((t, room)) = busy.pop().unwrap();
            let duration = end - start;
            let new_end = t + duration;
            count[room] += 1;
            busy.push(Reverse((new_end, room)));
        }
    }

    let mut best = 0usize;
    for room in 1..n {
        if count[room] > count[best] {
            best = room;
        }
    }
    best as i32
}
