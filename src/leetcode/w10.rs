pub fn is_match2(s: String, p: String) -> bool {
    // 將輸入字串和 pattern 轉成 char 向量，方便用索引存取
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();

    // dp[i][j] 表示 s[0..i] 是否能被 p[0..j] 匹配
    let mut dp = vec![vec![false; n + 1]; m + 1];
    // 初始條件：空字串和空 pattern 可以匹配
    dp[0][0] = true;

    // 處理 pattern 形如 a*, a*b*, a*b*c* 能匹配空字串的情況
    for j in 1..=n {
        if p[j - 1] == '*' {
            // '*' 可以讓前一個字元消失
            dp[0][j] = dp[0][j - 2];
        }
    }

    // 動態規劃填表
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                // 1. '*' 當作前一個元素出現0次
                dp[i][j] = dp[i][j - 2];
                // 2. 如果前一個 pattern 字元能匹配當前 s 字元，則 '*' 可以消耗一個 s 字元
                if matches_char(p[j - 2], s[i - 1]) {
                    dp[i][j] |= dp[i - 1][j];
                }
            } else {
                // 非 '*',只要當前字元能匹配就繼承左上角的狀態
                if matches_char(p[j - 1], s[i - 1]) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
    }
    dbg!(&dp);

    // 回傳最終結果
    dp[m][n]
}

// helper: does pattern character pc match string character sc?
fn matches_char(pc: char, sc: char) -> bool {
    pc == '.' || pc == sc
}

#[cfg(test)]
mod tests {
    use super::is_match;

    #[test]
    fn example1() {
        assert_eq!(is_match("aa".into(), "a".into()), false);
    }

    #[test]
    fn example2() {
        assert_eq!(is_match("aa".into(), "a*".into()), true);
    }

    #[test]
    fn example3() {
        assert_eq!(is_match("ab".into(), ".*".into()), true);
    }

    #[test]
    fn example4() {
        assert_eq!(is_match("ab".into(), "aabcd".into()), false);
    }

    #[test]
    fn more_cases() {
        assert!(is_match("aab".into(), "c*a*b".into())); // c* -> "", a* -> "aa"
        assert!(!is_match("mississippi".into(), "mis*is*p*.".into()));
    }
}

pub fn is_match(s: String, p: String) -> bool {
    // 將輸入字串和 pattern 轉成 char 向量，方便用索引存取
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();
    // dp[i][j] 表示 s[0..i] 是否能被 p[0..j] 匹配
    let mut dp = vec![vec![false; n + 1]; m + 1];
    // 初始條件：空字串和空 pattern 可以匹配
    dp[0][0] = true;

    // 處理 pattern 形如 a*, a*b*, a*b*c* 能匹配空字串的情況
    for j in 1..=n {
        if p[j - 1] == '*' {
            // '*' 可以讓前一個字元消失
            dp[0][j] = dp[0][j - 2];
        }
    }

    // 動態規劃填表
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                // 1. '*' 當作前一個元素出現0次
                dp[i][j] = dp[i][j - 2];
                // 2. 如果前一個 pattern 字元能匹配當前 s 字元，則 '*' 可以消耗一個 s 字元
                if match_char2(s[i - 1], p[j - 2]) {
                    dp[i][j] |= dp[i - 1][j]
                }
            } else {
                // 非 '*',只要當前字元能匹配就繼承左上角的狀態
                if match_char2(s[i - 1], p[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1]
                }
            }
        }
    }
    // 回傳最終結果
    dp[m][n]
}

pub fn match_char2(sc: char, pc: char) -> bool {
    sc == pc || pc == '.'
}
