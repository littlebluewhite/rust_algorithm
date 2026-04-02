// Input: str1 = "TFTF", str2 = "ab"
// Output: "ababa"

pub fn generate_string(str1: String, str2: String) -> String {
    let str1 = str1.as_bytes();
    let str2 = str2.as_bytes();
    let n = str1.len();
    let m = str2.len();
    let mut word = vec![0u8; n + m-1];
    let mut fixed = vec![false; n + m-1];
    for i in 0..n{
        if str1[i] != b'T' {
            continue;
        }
        for j in 0..m{
            if word[i+j] != 0 && word[i+j] != str2[j] {
                return String::new();
            }
            word[i+j] = str2[j];
            fixed[i+j] = true;
        }
    }

    for i in 0..(n+m-1){
        if word[i] == 0 {
            word[i] = b'a';
        }
    }

    for i in 0..n{
        if str1[i] != b'F' {
            continue;
        }
        let mut equal = true;
        for j in 0..m{
            if word[i+j] != str2[j]{
                equal = false;
                break;
            }
        }
        if !equal {
            continue;
        }

        let mut changed = false;
        for j in (0..m).rev(){
            if !fixed[i+j]{
                word[i+j] = b'b';
                changed = true;
                break;
            }
        }
        if !changed{
            return String::new();
        }
    }
    String::from_utf8(word).unwrap()
}