pub fn compare_version(version1: String, version2: String) -> i32 {
    let b1 = version1.as_bytes();
    let b2 = version2.as_bytes();
    let (mut i, mut j) = (0usize, 0usize);
    while i < b1.len() || j < b2.len() {
        let mut v1: i64 = 0;
        while i <b1.len() && b1[i] != b'.'{
            v1 = v1 *10 + (b1[i] - b'0') as i64;
            i += 1;
        }
        let mut v2: i64 = 0;
        while j < b2.len() && b2[j] != b'.'{
            v2 = v2 *10 + (b2[j] - b'0') as i64;
            j += 1;
        }
        if v1 > v2{
            return 1
        }else if v1 < v2{
            return -1
        }
        if i < b1.len(){
            i += 1;
        }
        if j < b2.len(){
            j += 1;
        }
    }
    0
}