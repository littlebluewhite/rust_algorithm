pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    fn sig(s: &str) -> [u8;26] {
        let mut cnt = [0u8;26];
        for b in s.bytes(){
            cnt[(b - b'a') as usize] += 1;
        }
        cnt
    }
    let mut result: Vec<String> = Vec::with_capacity(words.len());
    let mut prev: Option<[u8;26]> = None;
    for w in words{
        let cur = sig(&w);
        if prev.as_ref().map_or(true, |p| p != &cur){
            result.push(w);
            prev = Some(cur);
        }
    }
    result
}