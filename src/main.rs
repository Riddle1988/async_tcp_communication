mod command_line_parse;
use std::process;
use std::env;
use command_line_parse::Arguments;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg = match Arguments::parse(&args) {
        Ok(arguments) => arguments,
        Err(error) => {
            println!("\nError while parsing arguments: {}", error);
            Arguments::help();
            process::exit(1);
        }
    };
    println!("{:?}", arg);
}
