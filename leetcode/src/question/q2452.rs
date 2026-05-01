pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    // let qn = queries.len();
    // let dn = dictionary.len();
    // let mut ans: Vec<String> = Vec::with_capacity(qn);
    // for i in 0..qn{
    //     for j in 0.. dn{
    //         if within_tow_edit(queries[i].as_bytes(), dictionary[j].as_bytes()){
    //             ans.push(queries[i].clone());
    //             break;
    //         }
    //     }
    // }
    // ans
    queries
        .into_iter()
        .filter(|query| {
            dictionary
                .iter()
                .any(|word| within_tow_edit(query.as_bytes(), word.as_bytes()))
        })
        .collect()
}

fn within_tow_edit(q: &[u8], d: &[u8]) -> bool {
    let n = q.len();
    let mut count = 0;
    for i in 0..n {
        if q[i] != d[i] {
            count += 1;
        }
        if count > 2 {
            return false;
        }
    }
    true
}
