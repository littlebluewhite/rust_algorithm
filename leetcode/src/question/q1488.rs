use std::collections::{BTreeSet, HashMap};

pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let n = rains.len();
    let mut ans = vec![0;n];
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut dry_days: BTreeSet<usize> = BTreeSet::new();
    for (i, &lake) in rains.iter().enumerate(){
        if lake == 0 {
            ans[i] = 1;
            dry_days.insert(i);
        }else{
            if let Some(&pre) = last.get(&lake){
                if let Some(&k) = dry_days.range((pre+1)..).next(){
                    ans[k] = lake;
                    dry_days.remove(&k);
                }else{
                    return vec![];
                }
            }
            last.insert(lake, i);
        }
    }
    ans
}