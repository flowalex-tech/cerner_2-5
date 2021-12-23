extern crate rand;
use rand::Rng;
// cerner_2tothe5th_2021

fn main() {
    let mut yatzee_count = 0;
    for i in 1..=3 {
        let roll1 = rand::thread_rng().gen_range(1..6);
        let roll2 = rand::thread_rng().gen_range(1..6);
        let roll3 = rand::thread_rng().gen_range(1..6);
        let roll4 = rand::thread_rng().gen_range(1..6);
        let roll5 = rand::thread_rng().gen_range(1..6);

        if roll1 == roll2 && roll1 == roll3 && roll1 == roll4 && roll1 == roll5 {
            yatzee_count += 1;
            println!("YATZEE")
        }

        println!("{}, {}, {}, {}, {}", roll1, roll2, roll3, roll4, roll5)
    }

}
