use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
extern crate rpassword;
use rpassword::read_password;
// based off of this turtorial: http://saidvandeklundert.net/learn/2021-08-06-rust-ssh-cli-tool/
// cerner_2tothe5th_2021

fn main() {
    println!("Enter your password: ");
    let password = read_password().unwrap();
    //Change the IP to that of the host or enter the hostname/fqdn
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    // Update username with your user
    sess.userauth_password("flowalex", &password).unwrap();
    let mut channel = sess.channel_session().unwrap();
    // update command with the command you want to run
    channel.exec("uname -r").unwrap();
    let mut command_output = String::new();
    channel.read_to_string(&mut command_output).unwrap();
    println!("{}", command_output);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}
