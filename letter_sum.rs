use std::collections::HashMap;
use std::env;
// cerner_2tothe5th_2021
fn lettersum(word: &str, dict: &HashMap<char, usize>) -> usize {
    word.to_lowercase()
        .chars()
        .map(|ch| dict.get(&ch).unwrap_or(&0usize))
        .sum()
}

fn main() {
    let dict: HashMap<char, usize> = ('a'..='z')
        .enumerate()
        .map(|tuple| (tuple.1, tuple.0 + 1))
        .collect();

    let args = env::args().skip(1);
    let len = args.len();

    match len {
        0usize => println!("no argument provided"),
        _ => {
            for (ind, arg) in args.enumerate() {
                println!("{}. {} {}", ind + 1, &arg, lettersum(&arg, &dict));
            }
        }
    }

}
//
 fn tests() {
    assert_eq!(0, lettersum("", &Default::default()));
    assert_eq!(1, lettersum("a", &Default::default()));
    assert_eq!(26, lettersum("z", &Default::default()));
    assert_eq!(6, lettersum("cab", &Default::default()));
    assert_eq!(100, lettersum("excellent", &Default::default()));
    assert_eq!(317, lettersum("microspectrophotometries", &Default::default()));
 }
