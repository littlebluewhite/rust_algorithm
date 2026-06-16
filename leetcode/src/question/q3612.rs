pub fn process_str(s: String) -> String {
    let b = s.as_bytes();
    let mut bytes: Vec<u8> = Vec::new();
    for &i in b {
        match i {
            b'a'..=b'z' => {
                bytes.push(i);
            }
            b'#' => {
                bytes.extend(bytes.clone());
            }
            b'%' => {
                bytes.reverse();
            }
            b'*' => {
                bytes.pop();
            }
            _ => {}
        }
    }
    String::from_utf8(bytes).unwrap()
}
