extern crate clap;
use clap::App;
use std::io::{self, Write};
fn main() {
    let app = App::new("hello")
        .version("0.1")
        .author("Sean Hellum")
        .about("Hello world cli")
        .args_from_usage("-r --double 'Say hello'");

    let matches = app.get_matches();
    if matches.is_present("double") {
        double_me();
    }
}
fn double_me() {
    print!("What number do you want to double?: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    println!("{} doubled is {}",n,2*n);
}