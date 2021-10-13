use regex::Regex;
use regex::RegexSet;
// cerner_2tothe5th_2021

fn main() {
    // If you wrote all the numbers from 300 to 400 on a piece of paper, how many times would you have written the number 3?
    let set = RegexSet::new(&[
        r"\d[3]\d[0-9]\d[0-9]",
        r"\d[3]\d[3]\d[0-9]",
        r"\d[3]\d[0-9]\d[3]"
    ]).unwrap();
    for i in 300..400{
        //println!("{}", i);
        let mut count = 0;
        let x :String = i.to_string();
        for cap in set.into_iter()(&*x){
            println!("{}", &cap[1]);
            count += 1;
            println!("{}", count)
        }

    }
}
