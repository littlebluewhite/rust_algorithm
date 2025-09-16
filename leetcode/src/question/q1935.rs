pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let mut broken_mask: u32 = 0;
    for b in broken_letters.as_bytes(){
        broken_mask |= 1 << (b - b'a');
    }
    let mut ans = 0;
    'words: for w in text.split_whitespace(){
        for ch in w.bytes(){
            if (broken_mask & (1 << (ch -b'a'))) != 0{
                continue 'words;
            }
        }
        ans += 1;
    }
    ans
}