use async_std::net::{IpAddr};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Arguments {
    Mode(String),
    Address(async_std::net::IpAddr),
    Port(u16),
}

struct ValidArguments {
    description: String,
    argument_function: fn(&str) -> Result<Arguments, Box<dyn Error>>,
}
impl ValidArguments {
    fn options() -> HashMap<String, ValidArguments> {
        let mut options: HashMap<String, ValidArguments> = HashMap::new();
        options.insert(
            String::from("mode"),
            ValidArguments {
                description: String::from("client or server mode available (e.g. mode=client"),
                argument_function: Self::parse_mode,
            },
        );
        options.insert(
            String::from("address"),
            ValidArguments {
                description: String::from("valid server ip address (e.g. address=127.0.0.1"),
                argument_function: Self::parse_address,
            },
        );
        options.insert(
            String::from("port"),
            ValidArguments {
                description: String::from("valid server port (e.g. port=25768"),
                argument_function: Self::parse_port,
            },
        );
        options
    }

    fn parse_mode(mode_value: &str) -> Result<Arguments, Box<dyn Error>> {
        match mode_value {
            "client" | "server" => return Ok(Arguments::Mode(String::from(mode_value))),
            _ => return Err("Wrong mode type try with client or server")?,
        }
    }

    fn parse_address(address_value: &str) -> Result<Arguments, Box<dyn Error>> {
        return Ok(Arguments::Address((*address_value).parse::<IpAddr>()?));
    }

    fn parse_port(port_value: &str) -> Result<Arguments, Box<dyn Error>> {
        return Ok(Arguments::Port((*port_value).parse::<u16>()?));
    }
}

impl Arguments {
    pub fn parse(args: &Vec<String>) -> Result<Vec<Arguments>, Box<dyn Error>> {
        let mut arguments = Vec::new();
        let mut arg_options = ValidArguments::options();
        for arg in args {
            let (key, value) = Self::split_argument(arg)?;
            if arg_options.contains_key(&key) {
                let func = arg_options.remove(&key).unwrap().argument_function;
                arguments.push(func(&value)?);
            }
        }
        if arguments.is_empty() {
            Err("No Arguments found")?
        } else if !(arg_options.is_empty()){
            let unused_keys = Self::get_unset_arguments(&arg_options);
            Err(format!("{}is missing", unused_keys))?
        }
        else {
            Ok(arguments)
        }
    }

    fn split_argument(arg: &str) -> Result<(String, String), Box<dyn Error>> {
        let split_vector: Vec<&str> = arg.split("=").collect();
        if split_vector.len() != 2 {
            Err("Argument must have exactly one `=` character")?
        } else {
            Ok((
                split_vector[0].to_lowercase(),
                split_vector[1].to_lowercase(),
            ))
        }
    }

    fn get_unset_arguments(arg_options: &HashMap<String, ValidArguments>) -> String {
        let mut result: String = String::from("");
        for (key, _value) in arg_options.into_iter() {
            result.push_str(format!("[{}] ", key).as_ref());
        }
        result
    }

    pub fn help() {
        let arg_options = ValidArguments::options();
        println!("Help:\nexample: program.exe mode=server address=127.0.0.1 port=25786");
        for (key, value) in arg_options {
            println!("{}: {}", key, value.description);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn happy_pass() {
        let correct_object = vec![
            Arguments::Mode(String::from("client")),
            Arguments::Address(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            Arguments::Port(42),
        ];
        let test_args = vec![
            "mode=client".to_string(),
            "address=127.0.0.1".to_string(),
            "port=42".to_string(),
        ];

        let test_object = Arguments::parse(&test_args).unwrap();

        assert_eq!(correct_object, test_object)
    }
    #[test]
    fn without_arguments() {
        let test_args: Vec<String> = Vec::new();
        let result = Arguments::parse(&test_args);
        match result {
            Ok(_test_result) => assert!(false),
            Err(error) => {
                let test_error: Box<dyn Error> = String::from("No Arguments found").into();
                assert_eq!(error.to_string(), test_error.to_string());
            }
        }
    }

    #[test]
    fn wrong_mode_argument() {
        let test_args = vec![
            "mode=this_is_wrong".to_string(),
        ];
        let result = Arguments::parse(&test_args);
        match result {
            Ok(_test_result) => assert!(false),
            Err(error) => {
                let test_error: Box<dyn Error> = String::from("Wrong mode type try with client or server").into();
                assert_eq!(error.to_string(), test_error.to_string());
            }
        }
    }

    #[test]
    fn wrong_address_argument() {
        let test_args = vec![
            "address=this_is_wrong".to_string(),
        ];
        let result = Arguments::parse(&test_args);
        match result {
            Ok(_test_result) => assert!(false),
            Err(error) => {
                let test_error: Box<dyn Error> = String::from("invalid IP address syntax").into();
                assert_eq!(error.to_string(), test_error.to_string());
            }
        }
    }

    #[test]
    fn wrong_port_argument() {
        let test_args = vec![
            "port=this_is_wrong".to_string(),
        ];
        let result = Arguments::parse(&test_args);
        match result {
            Ok(_test_result) => assert!(false),
            Err(error) => {
                let test_error: Box<dyn Error> = String::from("invalid digit found in string").into();
                assert_eq!(error.to_string(), test_error.to_string());
            }
        }
    }

    #[test]
    fn no_separator() {
        let test_args = vec![
            "portthis_is_wrong".to_string(),
        ];
        let result = Arguments::parse(&test_args);
        match result {
            Ok(_test_result) => assert!(false),
            Err(error) => {
                let test_error: Box<dyn Error> = String::from("Argument must have exactly one `=` character").into();
                assert_eq!(error.to_string(), test_error.to_string());
            }
        }
    }
    #[test]
    fn two_separators() {
        let test_args = vec![
            "por=tthis_i=s_wrong".to_string(),
        ];
        let result = Arguments::parse(&test_args);
        match result {
            Ok(_test_result) => assert!(false),
            Err(error) => {
                let test_error: Box<dyn Error> = String::from("Argument must have exactly one `=` character").into();
                assert_eq!(error.to_string(), test_error.to_string());
            }
        }
    }
    #[test]
    fn lower_case_conversion() {
        let test_args = vec![
            "MODE=CLIENT".to_string(),
            "ADDRESS=127.0.0.1".to_string(),
            "PORT=42".to_string(),
        ];
        let result = Arguments::parse(&test_args);
        match result {
            Ok(test_result) => {
                match &test_result[0] {
                    Arguments::Mode(small_client) => {
                        assert_eq!(*small_client, String::from("client"))
                    },
                    _ => {
                        assert!(false)
                    }
                };
            },
            Err(_error) => {
                assert!(false)
            }
        }
    }

    #[test]
    fn to_few_arguments() {
        let test_args = vec![
            "mode=client".to_string(),
        ];
        let result = Arguments::parse(&test_args);
        match result {
            Ok(_test_result) => assert!(false),
            Err(error) => {
                let test_error: Box<dyn Error> = String::from("[address] [port] is missing").into();
                assert_eq!(error.to_string(), test_error.to_string());
            }
        }
    }
}
