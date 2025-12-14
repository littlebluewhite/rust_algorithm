use std::collections::HashMap;

pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
    let allowed = ["electronics", "grocery", "pharmacy", "restaurant"];
    let mut order: HashMap<&str, usize> = HashMap::new();
    for (idx, cat) in allowed.iter().enumerate() {
        order.insert(cat, idx);
    }

    let mut valid: Vec<(usize, String)> = Vec::new();

    for ((c, b), active) in code.into_iter().zip(business_line.into_iter()).zip(is_active.into_iter()) {
        if !active {
            continue;
        }
        if c.is_empty() {
            continue;
        }
        if !c.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            continue;
        }
        if let Some(&idx) = order.get(b.as_str()) {
            valid.push((idx, c));
        }
    }

    valid.sort_by(|a, b| a.cmp(b));
    valid.into_iter().map(|(_, code)| code).collect()
}