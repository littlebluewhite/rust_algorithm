pub fn make_largest_special(s: String) -> String {
    fn dfs(s: &str) -> String {
        let b = s.as_bytes();
        let mut balance = 0;
        let mut start = 0usize;
        let mut blocks: Vec<String> = Vec::new();
        for i in 0..b.len(){
            if b[i] == b'1'{
                balance +=1;
            }else{
                balance -=1;
            }
            if balance == 0{
                let inner = if i > start +1{
                    dfs(&s[start+1..i])
                }else {
                    String::new()
                };
                let mut block = String::with_capacity(i-start+1);
                block.push('1');
                block.push_str(&inner);
                block.push('0');
                blocks.push(block);
                start = i+1;
            }
        }
        blocks.sort_unstable_by(|a, b| b.cmp(a));
        blocks.concat()
    }
    dfs(&s)
}