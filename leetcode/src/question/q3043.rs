use std::collections::HashSet;

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    for i in 0..arr1.len(){
        let mut x = arr1[i];
        while x > 0{
            set.insert(x);
            x /= 10;
        }
    }
    let mut ans = 0;
    for i in 0..arr2.len(){
        let mut x = arr2[i];
        let mut len = digit_count(x);
        while x > 0{
            if set.contains(&x){
                ans = ans.max(len);
            }
            x /= 10;
            len -= 1;
        }
    }
    ans
}

fn digit_count(mut x: i32) -> i32 {
    let mut res = 0;
    while x > 0{
        x /= 10;
        res += 1;
    }
    res
}