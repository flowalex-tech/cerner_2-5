fn add_one(num: usize) -> usize {
    std::iter::repeat(())
        .scan(num, |n, _| if *n != 0 {let d = *n % 10 + 1; *n /= 10; Some(d)} else {None})
        .fold((0, 1), |(r, m), n| (n * m + r, m * if n == 10 {100} else {10}))
        .0
}
// cerner_2tothe5th_2021

fn main() {
    println!("add_one(42) -> {}", add_one(42));
    println!("add_one(111) -> {}", add_one(111));
    println!("add_one(123) -> {}", add_one(123));
    println!("add_one(998) -> {}", add_one(998));
    println!("add_one(999) -> {}", add_one(999));
}
