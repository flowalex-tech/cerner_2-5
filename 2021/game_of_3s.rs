use std::io;
// cerner_2tothe5th_2021
fn main() {
    //The input is a single number: the number at which the game starts.
    //Write a program that plays the Threes game, and outputs a valid sequence of steps you need to take to get to 1.
    // Each step should be output as the number you start at, followed by either -1 or 1 (if you are adding/subtracting 1 before dividing),
    // or 0 (if you are just dividing). The last line should simply be 1.
    println!("Enter any value");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: i32 = input.trim().parse().unwrap();
    while num != 1 {
        let check = num % 3;
        let check2 = (num +1) % 3;
        if check == 0 {
            println!("{} equals 0", num);
            let num_new = num / 3;
        } else if check2 == 0 {
            println!("{} +1", num);
            let num_new = num + 1;
            let num = num_new % 3;
        } else {
            println!("{} -1", num);
            let num_new = num - 1;
            let num = num_new % 3;
        }

    }
}
