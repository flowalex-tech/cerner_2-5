use std::fs::File;
use std::io::{self, BufRead, BufReader};
// cerner_2tothe5th_2021
fn count_words() -> Result<(), io::Error> {
    let line_counts =
        // Open file, wrap in a buffer.
        BufReader::new(File::open("file.txt")?)
            // Build line iterator from buffered file.
            .lines()
            // Split each line into words and count them.
            .map(|rl| rl.map(|l| l.split_whitespace().count()));

    // Sum line counts, aborting on error.
    let mut word_count = 0;
    for count in line_counts {
        word_count += count?;
    }

    println!("{}", word_count);
    Ok(())
}

fn main() {
    // Discard errors.
    count_words().unwrap();
}
