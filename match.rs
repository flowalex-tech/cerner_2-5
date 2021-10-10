// some older rust code I had lying around don't have much time today to create something
// cerner_2tothe5th_2021
fn main() {
    let some_bool = true;
    //let some_bool = false;
    match some_bool{
        true => println!("True"),
        false => println!("False"),
    }

    let some_int = 9;
    // let some_int  = 3
    match some_int {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("{:?}", some_int),
    }

    let my_name = "Alex";
    //let my_name = "Bob";
    //let my_name = "Adam";
    match my_name{
        "Alex" => println!("That is my name"),
        "Bob" => println!("That is not my name"),
        "Alice" => println!("Hello Alice"),
        _ => println!("Nice to meet you {:?}", my_name),
    }
}
