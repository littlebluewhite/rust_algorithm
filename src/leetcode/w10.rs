pub fn is_match2(s: String, p: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();

    // dp[i][j] = whether s[0..i] matches p[0..j]
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // Handle patterns like a*, a*b*, a*b*c* matching empty s
    for j in 1..=n {
        if p[j - 1] == '*' {
            // '*' can eliminate its preceding element
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                // 1) treat '*' as matching zero of preceding element
                dp[i][j] = dp[i][j - 2];
                // 2) if preceding element matches s[i-1], consume one s-char
                if matches_char(p[j - 2], s[i - 1]) {
                    dp[i][j] |= dp[i - 1][j];
                }
            } else {
                // direct match or '.' wildcard
                if matches_char(p[j - 1], s[i - 1]) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
    }
    dbg!(&dp);

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
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    //initial
    dp[0][0] = true;

    for j in 1..=n {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                dp[i][j] = dp[i][j - 2];
                if match_char2(s[i - 1], p[j - 2]) {
                    dp[i][j] |= dp[i - 1][j]
                }
            } else {
                if match_char2(s[i - 1], p[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1]
                }
            }
        }
    }
    dp[m][n]
}

pub fn match_char2(sc: char, pc: char) -> bool {
    sc == pc || pc == '.'
}
