extern crate clap;
use clap::{App,SubCommand,Arg};
use std::io::{self, Write};
fn main() {
    let app = App::new("RustCalc")
        .version("0.1")
        .author("Sean Hellum")
        .about("RustCalc cli")
        .args_from_usage("-r --double 'Multiply by 2'
                         -a --add 'add 2 numbers'")
        .subcommand(
            App::new("double")
                .about("doubles it")
                .author("Sean Hellum")
                .arg(
                    Arg::with_name("num")
                        .long("num")
                        .takes_value(true)
                        .multiple(false),
                ),
        )
        .subcommand(
            App::new("add")
                .about("adds 2 nums")
                .author("Sean Hellum")
                .arg(
                    Arg::with_name("num")
                        .long("num")
                        .takes_value(true)
                        .multiple(false),
                ),
        )
        .get_matches();
    if app.is_present("double") {
        double_me();
    }
    if app.is_present("add") {
        addNums();
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
fn addNums() {
    print!("What is the first number?: ");
    io::stdout().flush().unwrap();
    let mut item1 = String::new();
    std::io::stdin().read_line(&mut item1).unwrap();
    let item1: i32 = item1.trim().parse().unwrap();
    print!("What is the second number?: ");
    io::stdout().flush().unwrap();
    let mut item2 = String::new();
    std::io::stdin().read_line(&mut item2).unwrap();
    let item2: i32 = item2.trim().parse().unwrap();
    println!("The sum is {}",item1+item2);
}