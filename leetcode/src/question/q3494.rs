pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
    let n = skill.len();
    let m = mana.len();
    let mut p = vec![0i64; n + 1];
    for i in 0..n {
        p[i + 1] = p[i] + skill[i] as i64;
    }
    let b: Vec<i64> = mana.iter().map(|&x| x as i64).collect();
    let mut total_gap = 0;
    for j in 0..m.saturating_sub(1) {
        let b_j = b[j];
        let b_j1 = b[j + 1];
        let mut gap = i64::MIN;
        for i in 0..n {
            let cand = p[i+1] * b_j - p[i] * b_j1;
            if cand > gap {
                gap = cand;
            }
        }
        total_gap += gap;
    }
    total_gap + p[n] * b[m - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let skill = vec![1, 5, 2, 4];
        let mana = vec![5, 1, 4, 2];
        assert_eq!(min_time(skill, mana), 110);
    }

    #[test]
    fn example2() {
        let skill = vec![1, 1, 1];
        let mana = vec![1, 1, 1];
        assert_eq!(min_time(skill, mana), 5);
    }

    #[test]
    fn example3() {
        let skill = vec![1, 2, 3, 4];
        let mana = vec![1, 2];
        assert_eq!(min_time(skill, mana), 21);
    }
}
