use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
    let n = tasks.len();
    let mut ans = Vec::with_capacity(n);
    let mut free: BinaryHeap<Reverse<(i32, usize)>> = servers
        .iter()
        .enumerate()
        .map(|(index, &weight)| Reverse((weight, index)))
        .collect();
    let mut now = 0i32;
    let mut occupied:BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for i in 0..n{
        now = now.max(i as i32);
        if free.is_empty(){
            let Reverse((time, index)) = occupied.pop().unwrap();
            now = time;
            free.push(Reverse((servers[index], index)));
        }
        while let Some(&Reverse((time, index))) = occupied.peek(){
            if time > now{
                break;
            }
            occupied.pop();
            free.push(Reverse((servers[index], index)));
        }
        if let Some(Reverse((_, index))) = free.pop(){
            ans.push(index as i32);
            occupied.push(Reverse((tasks[i]+now, index)));
        }
    }
    ans
}
