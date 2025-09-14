use leetcode::question::q996;

fn main() {
    let wordlist = vec!["KiTe".to_string(), "kite".to_string(), "hare".to_string(), "Hare".to_string()];
    let queries = vec![
        "kite".to_string(), "KiTe".to_string(), "HARE".to_string(),
        "Hear".to_string(), "keti".to_string(), "keet".to_string(), "keto".to_string()
    ];
    let res = q996::spellchecker(wordlist, queries);
    println!("{:?}", res);
}
