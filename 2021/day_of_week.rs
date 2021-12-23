use chrono::{NaiveDate, Datelike, Weekday};
use std::io;
//Given the year, month, and day, return the day of the week.
//NOTE: This may have issues with trying to find the date after 1-20-2038 03:14:07 UTC on any processor using 32bit signed binary integer for time.  It has only been tested on X86_64 bit hardware
//Docuemntation for bug: https://en.wikipedia.org/wiki/Year_2038_problem
//Relevant XKCD: https://xkcd.com/607/
// cerner_2tothe5th_2021

fn main() {
    println!("Please enter a date in the following format YYYY-MM-DD");
    println!("YYYY:");
    let mut year_in = String::new();
    io::stdin().read_line(&mut year_in).unwrap();
    let year_num: i32 = year_in.trim().parse().unwrap();
    println!("MM:");
    let mut month_in = String::new();
    io::stdin().read_line(&mut month_in).unwrap();
    let month_num: u32 = month_in.trim().parse().unwrap();
    println!("DD:");
    let mut day_in = String::new();
    io::stdin().read_line(&mut day_in).unwrap();
    let day_num: u32 = day_in.trim().parse().unwrap();
    let date_in = NaiveDate::from_ymd(year_num, month_num, day_num);
    println!("{}",date_in.weekday());
    println!("{}", date_in)
}
