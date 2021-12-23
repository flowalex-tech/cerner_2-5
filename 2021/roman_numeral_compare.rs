use itertools::diff_with;
// cerner_2tothe5th_2021
fn roman_numeral(digit: char) -> u32 {
    match digit {
        'M' => 1000,
        'D' => 500,
        'C' => 100,
        'L' => 50,
        'X' => 10,
        'V' => 5,
        'I' => 1,
        _ => unimplemented!(),
    }
}

fn compare(first_num: &str, second_num: &str) -> bool {
    if let Some(diff) = diff_with(first_num.chars(), second_num.chars(), |x, y| x == y) {
        match diff {
            itertools::Diff::FirstMismatch(_, mut i, mut j) => roman_numeral(i.next().unwrap()) < roman_numeral(j.next().unwrap()),
            itertools::Diff::Shorter(_, _) => false,
            itertools::Diff::Longer(_, _) => true,
        }
    } else {
        false
    }
}

fn main() {
    assert_eq!(compare("I", "I") ,  false);
    assert_eq!(compare("I", "II") ,  true);
    assert_eq!(compare("II", "I") ,  false);
    assert_eq!(compare("V", "IIII") ,  false);
    assert_eq!(compare("MDCLXV", "MDCLXVI") ,  true);
    assert_eq!(compare("MM", "MDCCCCLXXXXVIIII") ,  false);
    assert_eq!(compare("MDCCCCLXXXXVIIII", "MM") ,  true);
}
