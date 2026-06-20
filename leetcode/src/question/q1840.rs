pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
    let r_len = restrictions.len()+1;
    let mut restrict: Vec<(i64, i64)> = Vec::with_capacity(r_len);
    restrict.push((1, 0));
    for r in restrictions {
        restrict.push((r[0] as i64, r[1] as i64));
    }
    restrict.sort_by_key(|r| r.0);
    for i in 1..r_len {
        let &(prev_pos, prev_heigh) = &restrict[i-1];
        let &(curr_pos, curr_heigh) = &restrict[i];
        if curr_pos-prev_pos+prev_heigh < curr_heigh{
            restrict[i] = (curr_pos, curr_pos-prev_pos+prev_heigh);
        }
    }

    for i in (0..r_len-1).rev(){
        let &(next_pos, next_heigh) = &restrict[i+1];
        let &(curr_pos, curr_heigh) = &restrict[i];
        if next_pos-curr_pos+next_heigh < curr_heigh{
            restrict[i] = (curr_pos, next_pos-curr_pos+next_heigh);
        }
    }

    let mut ans = 0i64;
    for i in 0..r_len-1{
        let &(curr_pos, curr_heigh) = &restrict[i];
        let &(next_pos, next_heigh) = &restrict[i+1];
        ans = ans.max((curr_heigh+next_heigh+next_pos-curr_pos)/2);
    }

    let (last_restrict_pos, last_restrict_heigh) = restrict[r_len-1];
    ans = ans.max(n as i64-last_restrict_pos+last_restrict_heigh);

    ans as i32
}