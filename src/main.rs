extern crate clap;
use clap::App;

fn main() {
    let app = App::new("calc")
        .version("0.1")
        .author("Sean Hellum")
        .about("Calculator made in rust")
        .args_from_usage("-r --greet 'Say hello'");

    let matches = app.get_matches();
    if matches.is_present("greet") {
        greet_me();
    }
}
fn greet_me() {
    println!("Hello world from an arg");
}