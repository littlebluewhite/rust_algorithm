use std::collections::HashMap;

pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    let mut exact: HashMap<String, String> = HashMap::new();
    let mut case_insensitive: HashMap<String, String> = HashMap::new();
    let mut vowel_insensitive: HashMap<String, String> = HashMap::new();

    for word in wordlist {
        exact.entry(word.clone()).or_insert(word.clone());
        case_insensitive
            .entry(word.to_lowercase())
            .or_insert(word.clone());
        vowel_insensitive
            .entry(devowels(&word))
            .or_insert(word.clone());
    }

    queries
        .into_iter()
        .map(|query| {
            if let Some(ans) = exact.get(&query) {
                ans.clone()
            } else {
                if let Some(ans) = case_insensitive.get(&query.to_lowercase()) {
                    ans.clone()
                } else {
                    if let Some(ans) = vowel_insensitive.get(&devowels(&query)) {
                        ans.clone()
                    } else {
                        "".to_string()
                    }
                }
            }
        })
        .collect()
}

fn devowels(s: &str) -> String {
    s.chars()
        .map(|c| match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => '*',
            other => other,
        })
        .collect()
}
