use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    // 若 words 為空，直接回傳空陣列
    if words.is_empty() {
        return result;
    }

    // 1. 計算各種長度／次數
    let s_len = s.len();
    let word_count = words.len(); // words 中一共有多少個單詞
    let word_len = words[0].len(); // 每個單詞長度（題目保證都一樣）
    let total_len = word_len * word_count; // 總共要拼接的長度

    // 如果 s 太短，無法放下 total_len，就直接回空
    if s_len < total_len {
        return result;
    }

    // 2. 把 words 內每個單詞的「目標出現次數」放到 target_map
    //    key: &str（指向 words 裡的字串切片），value: 出現次數
    let mut target_map: HashMap<&str, i32> = HashMap::new();
    for w in &words {
        *target_map.entry(w.as_str()).or_insert(0) += 1;
    }

    // 方便反覆切字串，我們先把 s 轉成 &str
    let s_str = s.as_str();

    // 3. 外層 offset 迴圈，從 0 到 word_len-1
    //    意思是把 s 分成 word_len 條「字詞流」
    for offset in 0..word_len {
        // left, right 都指向「字節索引」(byte index)
        let mut left = offset;
        let mut right = offset;
        let mut count = 0; // 目前視窗內一共吃了多少個 word

        // 這張 map 用來維護「當前滑動視窗內」各詞出現了幾次
        let mut window_map: HashMap<&str, i32> = HashMap::new();

        // 只要 right 還能夠取到一個長度為 word_len 的子串，就繼續
        while right + word_len <= s_len {
            // 取出下一個候選詞：s[right..right+word_len]
            let word = &s_str[right..right + word_len];

            if target_map.contains_key(word) {
                // ① 如果這個 word 是我們想要的
                //    把它加到 window_map 裡，count + 1
                *window_map.entry(word).or_insert(0) += 1;
                count += 1;

                // ② 如果「某個詞的次數」已經超過了 target_map 裡的允許次數
                //    就必須一直收縮「左邊界」，直到不再超標
                while window_map.get(word).unwrap() > target_map.get(word).unwrap() {
                    // 取最左邊那個要踢掉的單詞
                    let left_word = &s_str[left..left + word_len];
                    // window_map 裡對應次數減 1
                    *window_map.get_mut(left_word).unwrap() -= 1;
                    // 視窗內單詞總數減 1
                    count -= 1;
                    // 左邊界右移一格(即 +word_len)
                    left += word_len;
                }

                // ③ 如果目前「視窗內剛好吃到 word_count 個單詞」
                //    則代表長度 = total_len，且 window_map[key] <= target_map[key] 對所有 key 都成立
                if count == word_count {
                    // 把 left（byte index）轉成 i32 放進結果
                    result.push(left as i32);

                    // 然後再把「最左邊那個單詞」踢掉，讓視窗往右滑一個單詞長度
                    let left_word = &s_str[left..left + word_len];
                    *window_map.get_mut(left_word).unwrap() -= 1;
                    count -= 1;
                    left += word_len;
                }
            } else {
                // ④ 如果這個 word 根本不在 target_map 中，
                //    那麼從 right 這裡開始，任何包含它的視窗都不可能合法
                //    直接把 window_map 清空、count 歸零，左邊界跳到 right+word_len
                window_map.clear();
                count = 0;
                left = right + word_len;
            }

            // 不管上述任何分支，都要把右邊界往右滑一格
            right += word_len;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    // 幫助函式：將兩個 Vec<i32> 先排序，再比較是否相等
    fn assert_vec_eq_unordered(mut got: Vec<i32>, mut expected: Vec<i32>) {
        got.sort_unstable();
        expected.sort_unstable();
        assert_eq!(got, expected);
    }

    #[test]
    fn test_example1() {
        // 範例 1：
        // s = "barfoothefoobarman", words = ["foo","bar"]
        // 可拼出的子串為 "barfoo" (index=0) 或 "foobar" (index=9)
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let mut result = find_substring(s, words);
        assert_vec_eq_unordered(result, vec![0, 9]);
    }

    #[test]
    fn test_example2_empty_result() {
        // 範例 2：沒有符合條件的拼接
        // s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        let mut result = find_substring(s, words);
        // 沒有任何拼接子串
        assert!(result.is_empty());
    }

    #[test]
    fn test_example3_multiple_offsets() {
        // 範例 3：
        // s = "barfofoobarthefoobarman", words = ["bar","foo","the"]
        // 注意這裡字串稍微改動，還是符合多個 offset 能夠找到
        // 從 index=5："foobarthe"
        //         8："barthefoo"
        //        11："thefoobar"
        let s = "barfofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let mut result = find_substring(s, words);
        assert_vec_eq_unordered(result, vec![5, 8, 11]);
    }

    #[test]
    fn test_empty_words() {
        // 當 words 為空時，直接回傳空結果
        let s = "anything".to_string();
        let words: Vec<String> = vec![];
        let result = find_substring(s, words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_s_shorter_than_total_len() {
        // 當 s 的長度比 word_len * word_count 還小，無法拼接
        let s = "short".to_string();
        let words = vec!["longword".to_string(), "another".to_string()];
        let result = find_substring(s, words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_repeated_words_in_words() {
        // words 中有重複字串的情形
        // s = "aaaaaa", words = ["aa","aa","aa"]
        // 所需 total_len = 2 * 3 = 6，整個 s 恰好可以從 index=0 拼成三個 "aa"
        let s = "aaaaaa".to_string();
        let words = vec!["aa".to_string(), "aa".to_string(), "aa".to_string()];
        let mut result = find_substring(s, words);
        assert_vec_eq_unordered(result, vec![0]);
    }

    #[test]
    fn test_overlapping_matches() {
        // 測試有重疊匹配的情況
        // s = "aaaabaaaab", words = ["aa","aa","aa"]
        // word_len=2, word_count=3 => total_len=6
        // s 作 2-byte 分割的 offset = 0 can 取到 index=0: "aaaaba" -> 切 ["aa","aa","ba"] (不符合)
        // 實際上，在這個例子中沒有合法拼接，期望空結果
        let s = "aaaabaaaab".to_string();
        let words = vec!["aa".to_string(), "aa".to_string(), "aa".to_string()];
        let result = find_substring(s, words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_non_overlapping_multiple() {
        // 測試多組不重疊的結果
        // s = "catfoxcatfox", words = ["cat","fox"]
        // 從 index=0："catfox"，從 index=3："foxcat"，從 index=6："catfox"，從 index=9："fox"→不滿長度
        // 所以合法起點：[0, 3, 6]
        let s = "catfoxcatfox".to_string();
        let words = vec!["cat".to_string(), "fox".to_string()];
        let mut result = find_substring(s, words);
        assert_vec_eq_unordered(result, vec![0, 3, 6]);
    }

    #[test]
    fn test_partial_overlap_invalid() {
        // 測試部分重疊但不合法的情況
        // s = "barfoofoo", words = ["bar","foo","foo"]
        // word_len=3, word_count=3 => total_len=9
        // 從 index=0："barfoofoo" -> 切分 ["bar","foo","foo"]，合法
        // 只有 index=0 符合
        let s = "barfoofoo".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "foo".to_string()];
        let mut result = find_substring(s, words);
        assert_vec_eq_unordered(result, vec![0]);
    }

    #[test]
    fn test_single_character_words() {
        // 測試當所有 words 都是單一字元的情況
        // s = "ababab", words = ["a","b","a"]
        // word_len=1, word_count=3 => total_len=3
        // s 中符合的起點：0:"aba"、1:"bab" (不符合，因為 counts 不對)、
        //                  2:"aba"、3:"bab" (不符合)，
        // 結果：0, 2
        let s = "ababab".to_string();
        let words = vec!["a".to_string(), "b".to_string(), "a".to_string()];
        let mut result = find_substring(s, words);
        assert_vec_eq_unordered(result, vec![0, 2]);
    }
}

pub fn find_substring2(s: String, words: Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if words.is_empty() {
        return result;
    }

    let s_len = s.len();
    let word_count = words.len();
    let word_len = words[0].len();
    let total_len = word_len * word_count;

    if total_len > s_len {
        return result;
    }

    let mut target_map: HashMap<&str, i32> = HashMap::new();
    for w in &words {
        *target_map.entry(w.as_str()).or_insert(0) += 1;
    }

    let s_str = s.as_str();

    for offset in 0..word_len {
        let mut left = offset;
        let mut right = offset;
        let mut count = 0;
        let mut window_map: HashMap<&str, i32> = HashMap::new();
        while right + word_len <= s_len {
            let word = &s_str[right..right + word_len];
            if target_map.contains_key(word) {
                *window_map.entry(word).or_insert(0) += 1;
                count += 1;
                while target_map.get(word) < window_map.get(word) {
                    *window_map.get_mut(&s_str[left..left+word_len]).unwrap() -= 1;
                    count -= 1;
                    left += word_len;
                }
                if count == word_count{
                    result.push(left as i32);
                    *window_map.get_mut(&s_str[left..left+word_len]).unwrap() -= 1;
                    count -=1;
                    left += word_len;
                }
            }else{
                window_map.clear();
                count = 0;
                left = right+word_len;
            }
            right += word_len;
        }
    }

    result
}
