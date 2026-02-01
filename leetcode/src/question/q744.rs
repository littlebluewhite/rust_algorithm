pub fn next_greatest_letter(mut letters: Vec<char>, target: char) -> char {
    let mut ans = letters[0];
    letters.sort_unstable();
    for letter in letters{
        if letter > target{
            ans = letter;
            break;
        }
    }
    ans
}