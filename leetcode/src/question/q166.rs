use std::collections::HashMap;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return "0".to_string()
    }
    let mut ans = String::new();
    let mut nagitive = false;
    let mut n = numerator as i128;
    let mut d = denominator as i128;
    let mut nagitive = (n < 0) ^ (d < 0);
    if n < 0 {
        n = -n;
    }
    if d < 0 {
        d = -d;
    }
    if nagitive {
        ans.push('-');
    }
    let int_part = n/d;
    let mut rem = n%d;
    ans.push_str(&int_part.to_string());
    if rem == 0 {
        return ans
    }
    ans.push('.');

    let mut seen: HashMap<i128, usize> = HashMap::new();
    while rem != 0 {
        if let Some(&pos) = seen.get(&rem){
            ans.insert(pos, '(');
            ans.push(')');
            break;
        }
        seen.insert(rem, ans.len());
        rem *= 10;
        let div = rem/d;
        rem = rem %d;
        ans.push(char::from(b'0'+(div as u8)));
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_cases() {
        assert_eq!(fraction_to_decimal(1, 2), "0.5");
        assert_eq!(fraction_to_decimal(2, 1), "2");
        assert_eq!(fraction_to_decimal(1, 5), "0.2");
        assert_eq!(fraction_to_decimal(1, 6), "0.1(6)");
        assert_eq!(fraction_to_decimal(4, 333), "0.(012)");
    }

    #[test]
    fn sign_and_large_range() {
        assert_eq!(fraction_to_decimal(-50, 8), "-6.25");
        assert_eq!(fraction_to_decimal(7, -12), "-0.58(3)");
        // 極端值：i32::MIN / 1 仍能安全處理（靠 i128）
        assert_eq!(fraction_to_decimal(i32::MIN, 1), "-2147483648");
        // 循環長一些的情況
        assert_eq!(fraction_to_decimal(1, 7), "0.(142857)");
    }

    #[test]
    fn zero_and_trivial() {
        assert_eq!(fraction_to_decimal(0, 7), "0");
        assert_eq!(fraction_to_decimal(1, 1), "1");
        assert_eq!(fraction_to_decimal(10, 2), "5");
    }
}
