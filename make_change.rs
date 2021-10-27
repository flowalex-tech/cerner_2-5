// cerner_2tothe5th_2021

fn change(mut n: i32) -> i32 {
    let coin_values: [i32; 6] = [500, 100, 25, 10, 5, 1];
    let mut sum = 0;

    for coin_value in coin_values.iter() {
        while n - coin_value >= 0 {
            n -= coin_value;
            sum += 1;
        }
    }
    sum
}

fn main () {
    let result = change(0);
    println!("{}", result);
    let result = change(12);
    println!("{}", result);
    let result = change(468);
    println!("{}", result);
    let result = change(123456);
    println!("{}", result);
}
