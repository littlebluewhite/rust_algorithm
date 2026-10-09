pub fn min_insertions(s: String) -> i32 {
    let mut insertions = 0i32;
    let mut need = 0i32;

    for &byte in s.as_bytes() {
        if byte == b'(' {
            if need % 2 == 1 {
                // 先完成連續的右括號對，才能讀入新的左括號。
                insertions += 1;
                need -= 1;
            }
            need += 2;
        } else {
            need -= 1;
            if need == -1 {
                // 補一個左括號；目前右括號已消耗它的一個需求。
                insertions += 1;
                need = 1;
            }
        }
    }

    insertions + need
}