// cerner_2tothe5th_2021

const ALPHABET: [&str; 26] = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."];

fn smorse(input: &str) -> String {
    let mut output_string: String = String::new();

    for c in input.to_lowercase().as_bytes() {
        if *c >= 97 && *c <= 122 {
            output_string.push_str(ALPHABET[(c - 97) as usize]);
        }
    };

    output_string
}

fn main() {
    let result = smorse("Hello World");
    println!("{}", result);
}
