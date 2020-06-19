mod command_line_parse;
use command_line_parse::Arguments;
use std::env;
use std::process;
mod client;
mod server;

fn main() {
    let args = match Arguments::parse(&(env::args().skip(1).collect())) {
        Ok(arguments) => arguments,
        Err(error) => {
            println!("\nError while parsing arguments: {}", error);
            Arguments::help();
            process::exit(1);
        }
    };
    let mode = args
        .iter()
        .filter_map(|f| {
            if let Arguments::Mode(ref value) = *f {
                Some(value)
            } else {
                None
            }
        })
        .next()
        .map(ToOwned::to_owned);

    let result = match mode.as_deref() {
        Some("client") => client::main(&args),
        Some("server") => server::main(&args),
        _ => panic!("Wrong type"),
    };
    match result {
        Ok(_) => (),
        Err(error) => {
            println!("\nError while running program: {}", error);
            Arguments::help();
            process::exit(1);
        }
    }

    println!("{:?}", args);
}
