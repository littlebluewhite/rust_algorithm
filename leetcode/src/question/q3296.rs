use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_number_of_seconds(mut mountain_height: i32, worker_times: Vec<i32>) -> i64 {
    let n = worker_times.len();
    let worker_times: Vec<i64> = worker_times.iter().map(|x| *x as i64).collect();
    let mut used_times = vec![0;n];
    let mut total_time = vec![0i64;n];
    let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    for i in 0..n{
        heap.push(Reverse((worker_times[i], i)));
    }
    while mountain_height >0{
        let Reverse((time, index)) = heap.pop().unwrap();
        total_time[index] = time as i64;
        used_times[index] += 1;
        mountain_height -= 1;
        let new_time = worker_times[index] * (used_times[index]+1);
        heap.push(Reverse((time + new_time, index)));
    }
    *total_time.iter().max().unwrap()
}