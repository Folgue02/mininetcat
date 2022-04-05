use std::env;
mod lib;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.contains(&String::from("--help")) {
        println!("\
MiniNetcat
    Description:
        MiniNetcat it's a basic utility that allows the user to connect 
        to a socket and send strings encoded in utf-8 that are prompted 
        through stdin.

    Usage:
        mininetcat {{targetip}} {{targetport}}");
    }
    match lib::run(&arguments) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("An error has occurred: {}", err);
            std::process::exit(1);
        }
    };
}
