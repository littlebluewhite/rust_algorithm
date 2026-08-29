pub fn lex_palindromic_permutation(s: String, target: String) -> String {
    let n = s.len();
    let half_len = n / 2;
    let target_bytes = target.as_bytes();
    let mut total = [0; 26];
    for c in s.chars() {
        total[(c as u8 - b'a') as usize] += 1;
    }
    let mut odd_count = 0;
    let mut middle = None;
    let mut half_freq = [0; 26];
    for c in 0..26 {
        if total[c] % 2 == 1 {
            odd_count += 1;
            middle = Some(c as u8 + b'a');
        }
        half_freq[c] = total[c] / 2;
    }
    if odd_count > 1 {
        return "".to_string();
    }
    let target_half = &target_bytes[..half_len];
    if can_build_half(&half_freq, target_half) {
        let candidate = build_palindrome(target_half, middle);
        if candidate.as_bytes() > target_bytes {
            return candidate;
        }
    }
    if let Some(next_half) = next_half_greater(&half_freq, target_half) {
        return build_palindrome(&next_half, middle);
    }
    "".to_string()
}

fn can_build_half(half_count: &[i32; 26], target_half: &[u8]) -> bool {
    let mut count = *half_count;
    for &c in target_half.iter() {
        if count[(c - b'a') as usize] > 0 {
            count[(c - b'a') as usize] -= 1;
        } else {
            return false;
        }
    }
    true
}

fn build_palindrome(half_target: &[u8], middle: Option<u8>) -> String {
    let mut ans = Vec::with_capacity(half_target.len() * 2 + usize::from(middle.is_some()));
    ans.extend_from_slice(half_target);
    if let Some(c) = middle {
        ans.push(c);
    }
    ans.extend(half_target.iter().rev().copied());
    String::from_utf8(ans).unwrap()
}

fn next_half_greater(half_count: &[i32; 26], target_half: &[u8]) -> Option<Vec<u8>> {
    let len = target_half.len();
    if len == 0 {
        return None;
    }

    let mut count = *half_count;
    let mut matched = 0usize;

    while matched < len {
        let idx = (target_half[matched] - b'a') as usize;
        if count[idx] == 0 {
            break;
        }
        count[idx] -= 1;
        matched += 1;
    }

    let mut pivot = if matched == len {
        let last_idx = (target_half[len - 1] - b'a') as usize;
        count[last_idx] += 1;
        len - 1
    } else {
        matched
    };

    loop {
        let target_idx = (target_half[pivot] - b'a') as usize;
        for bigger in target_idx + 1..26 {
            if count[bigger] == 0 {
                continue;
            }

            let mut half = Vec::with_capacity(len);
            half.extend_from_slice(&target_half[..pivot]);
            half.push(b'a' + bigger as u8);
            count[bigger] -= 1;

            for letter in 0..26 {
                for _ in 0..count[letter] {
                    half.push(b'a' + letter as u8);
                }
            }

            return Some(half);
        }

        if pivot == 0 {
            break;
        }

        let previous_idx = (target_half[pivot - 1] - b'a') as usize;
        count[previous_idx] += 1;
        pivot -= 1;
    }

    None
}
