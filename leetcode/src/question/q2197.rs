pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a.abs()
    }
    #[inline]
    fn lcm(a: i64, b: i64, g: i64) -> i64 {
        // a / gcd(a,b) * b
        (a / g) * b
    }

    let mut st: Vec<i64> = Vec::with_capacity(nums.len());
    for num in nums {
        let mut x = num as i64;

        // 不斷與前一個元素嘗試合併，直到互質或堆疊空
        loop {
            if let Some(&last) = st.last() {
                let g = gcd(last, x);
                if g > 1 {
                    st.pop();
                    x = lcm(last, x, g);
                    // 繼續往前試
                    continue;
                }
            }
            st.push(x);
            break;
        }
    }

    st.into_iter().map(|v| v as i32).collect()
}
