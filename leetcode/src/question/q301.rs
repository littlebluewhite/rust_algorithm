use std::collections::HashSet;

pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    fn dfs(
        bytes: &[u8],
        index: usize,
        remove_left: usize,
        remove_right: usize,
        balance: usize,
        path: &mut String,
        answer: &mut HashSet<String>,
    ) {
        if index == bytes.len() && remove_left == 0 && remove_right == 0 && balance == 0 {
            answer.insert(path.clone());
            return;
        }
        let remaining = bytes.len() - index;
        let need_remove = remove_left + remove_right;
        if need_remove > remaining || balance > remaining - need_remove {
            return;
        }
        match bytes[index] {
            b'(' => {
                if remove_left > 0 {
                    dfs(
                        bytes,
                        index + 1,
                        remove_left - 1,
                        remove_right,
                        balance,
                        path,
                        answer,
                    );
                }
                path.push('(');
                dfs(
                    bytes,
                    index + 1,
                    remove_left,
                    remove_right,
                    balance + 1,
                    path,
                    answer,
                );
                path.pop();
            }
            b')' => {
                if remove_right > 0 {
                    dfs(
                        bytes,
                        index + 1,
                        remove_left,
                        remove_right - 1,
                        balance,
                        path,
                        answer,
                    );
                }
                if balance > 0 {
                    path.push(')');
                    dfs(
                        bytes,
                        index + 1,
                        remove_left,
                        remove_right,
                        balance - 1,
                        path,
                        answer,
                    );
                    path.pop();
                }
            }
            letter => {
                path.push(letter as char);
                dfs(
                    bytes,
                    index + 1,
                    remove_left,
                    remove_right,
                    balance,
                    path,
                    answer,
                );
                path.pop();
            }
        }
    }

    let mut answer = HashSet::new();
    let b = s.as_bytes();
    let mut remove_left = 0usize;
    let mut remove_right = 0usize;
    for i in 0..b.len() {
        match b[i] {
            b'(' => remove_left += 1,
            b')' if remove_left > 0 => remove_left -= 1,
            b')' => remove_right += 1,
            _ => {}
        }
    }
    let mut path = String::new();
    dfs(
        s.as_bytes(),
        0,
        remove_left,
        remove_right,
        0,
        &mut path,
        &mut answer,
    );
    answer.into_iter().collect()
}
