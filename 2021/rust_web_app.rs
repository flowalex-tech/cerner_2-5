#[macro_use] extern crate nickel;
// Trying out the rust web server
// To run download this direcotry and use the command cargo run
// https://gitlab.com/flowalex/code-practice/-/tree/rust/rust/simple-log
// cerner_2tothe5th_2021
use nickel::Nickel;

fn say_hello() -> &'static str {
    "Hello world!"
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server.listen("127.0.0.1:6767");
}
