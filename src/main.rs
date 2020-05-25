mod command_line_parse;

use std::env;
use command_line_parse::ParsedArguments;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let test = ParsedArguments::parse(&args);
    println!("{:?}", test);
    // let mut args = env::args().skip(1).peekable(); // Iteratot that can look at the next element of the iterator without consuming it (as a reference)

    // match args.peek().map(|x| x.as_ref()) {
    //     Some("--option1") => {
    //          args.next(); // Skip the flag
    //     }
    //     _ => {
    //         println!("handle no option");
    //     }
    // }
}
