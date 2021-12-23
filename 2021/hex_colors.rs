use std::io;
// Given three integers between 0 and 255, corresponding to the red, green, and blue channel values of a color, find the hex string for that color.
// You may use anything built into your programming language, such as for base conversion, but you can also do it manually.
// cerner_2tothe5th_2021

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}",
            r as f32 as u8,
            g as f32 as u8,
            b as f32 as u8)
}

fn main() {
    println!("Please enter a RGB color code");
    println!("Red:");
    let mut red_in = String::new();
    io::stdin().read_line(&mut red_in).unwrap();
    let r: i32 = red_in.trim().parse().unwrap();
    println!("Green:");
    let mut green_in = String::new();
    io::stdin().read_line(&mut green_in).unwrap();
    let g: i32 = green_in.trim().parse().unwrap();
    println!("Blue:");
    let mut blue_in = String::new();
    io::stdin().read_line(&mut blue_in).unwrap();
    let b: i32 = blue_in.trim().parse().unwrap();

    println!("rgb {} {} {} to hex {} ", r, g, b, rgb(r, g, b));
}
