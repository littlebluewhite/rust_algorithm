pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut ans = Vec::new();
    let n = n as usize;
    fn dfs (n: usize, open: usize, close: usize, current: &mut String, res: &mut Vec<String>){
        if open == n && close == n {
            res.push(current.clone());
            return;
        }
        if open < n{
            current.push('(');
            dfs(n, open + 1, close, current, res);
            current.pop();
        }
        if close < open{
            current.push(')');
            dfs(n, open, close + 1, current, res);
            current.pop();
        }
    }
    dfs(n, 0, 0, &mut String::new(), &mut ans);
    ans
}