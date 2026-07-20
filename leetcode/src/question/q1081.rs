pub fn smallest_subsequence(s: String) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut last = vec![0;26];
    for (i, &c) in b.iter().enumerate() {
        last[(c - b'a') as usize] = i;
    }
    let mut in_stack = vec![false;26];
    let mut stack: Vec<u8> = Vec::with_capacity(n);
    for (i, &c) in b.iter().enumerate() {
        let idx = (c-b'a') as usize;
        if in_stack[idx] {
            continue;
        }
        while let Some(&top) = stack.last() {
            let t = (top - b'a') as usize;
            if t > idx && last[t] > i{
                stack.pop();
                in_stack[t] = false;
            } else {
                break;
            }
        }
        stack.push(c);
        in_stack[idx] = true;
    }
    String::from_utf8(stack).unwrap()
}