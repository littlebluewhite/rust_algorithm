pub fn process_str(s: String, k: i64) -> char {
    let b = s.as_bytes();
    let n = b.len();
    let mut lens = vec![0i128;n+1];
    for i in 0..n{
        let len = lens[i];
        lens[i+1] = {
            match b[i] {
                b'#' => len << 1,
                b'%' => len,
                b'*' => {
                    if len > 0{
                        len -1
                    } else {
                        0
                    }
                },
                _ => len + 1,
            }
        }
    }
    let mut k = k as i128;
    if k >= lens[n] || k < 0{
        return '.';
    }
    for i in (0..n).rev(){
        let pre = lens[i];
        let cur = lens[i+1];
        match b[i]{
            b'#' =>{
                if k >= pre{
                    k -= pre;
                }
            },
            b'%' =>{
                k = cur-1-k;
            },
            b'*' =>{

            }
            c =>{
                if pre == k{
                    return c as char;
                }
            }
        }
    }
    '.'
}