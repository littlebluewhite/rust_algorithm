pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|interval| interval[1]);
    println!("{:?}", intervals);
    let mut ans_list: Vec<i32> = Vec::new();
    ans_list.push(intervals[0][1]-1);
    ans_list.push(intervals[0][1]);
    for i in 1..intervals.len(){
        let mut count = 2;
        for j in ans_list.len()-2..ans_list.len(){
            if ans_list[j]>=intervals[i][0]&&ans_list[j]<=intervals[i][1]{
                count -=1;
            }
        }
        let mut num = intervals[i][1];
        let mut insert = Vec::new();
        while count > 0{
            if !ans_list.contains(&num){
                insert.push(num);
                count -=1;
            }
            num -=1;
        }
        while let Some(num) = insert.pop(){
            ans_list.push(num);
        }
    }
    println!("{:?}", ans_list);
    ans_list.len() as i32
}