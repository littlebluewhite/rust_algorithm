pub fn count_palindromic_subsequence(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut left = vec![n;26];
    let mut right = vec![0;26];
    let mut seen = vec![false;26];
    for i in 0..b.len(){
        let c = (b[i] -b'a') as usize;
        if !seen[c]{
            left[c] = i;
            seen[c] = true;
        }else{
            right[c] = i;
        }
    }
    let mut ans = 0;
    for i in 0..26{
        let mut middle = vec![false;26];
        if left[i]<right[i]{
            for j in left[i]+1 ..right[i]{
                let y = b[j]-b'a';
                middle[y as usize] = true;
            }
        }
        ans += middle.iter().filter(|&&x|x).count() as i32;
    }

    ans
}