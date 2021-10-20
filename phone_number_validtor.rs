extern crate phonenumber;
// cerner_2tothe5th_2021
use std::env;

fn main() {
// expected input +1-866-221-8877 or 18662218877
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() < 1 {
        panic!("not enough arguments");
    }

    let number  = args.pop().unwrap();
    let country = args.pop().map(|c| c.parse().unwrap());

    let phone = phonenumber::parse(country, number).unwrap();
    let valid  = phonenumber::is_valid(&phone);

    if valid == true {
        println!("{}", phone);
    }
}
