use async_std::net::{IpAddr, Ipv4Addr};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct Arguments {
    pub mode: String,
    pub address: async_std::net::IpAddr,
    pub port: u16,
}

struct Argument {
    description: String,
    argument_function: fn(&str) -> Result<Argument, Box<dyn Error>>,
}

impl Arguments {
    pub fn parse(args: &Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut arg_options = Self::options();
        for arg in args {
            let (key, value) = Self::split_argument(arg);
            if arg_options.contains_key(&key) {
                let func = arg_options.get(&key).unwrap().argument_function;
                func(&value)?;
            }
        }
        Err("Not Implemented")?
        // Ok(Arguments {
        //     mode,
        //     address,
        //     port,
        // })
    }

    fn split_argument(arg: &str) -> (String, String) {
        let split_vector: Vec<&str> = arg.split("=").collect();
        if split_vector.len() != 2 {
            Self::help();
        }
        (
            split_vector[0].to_lowercase(),
            split_vector[1].to_lowercase(),
        )
    }

    fn options() -> HashMap<String, Argument> {
        let mut options: HashMap<String, Argument> = HashMap::new();
        options.insert(
            String::from("mode"),
            Argument {
                description: String::from("Help text"),
                argument_function: Self::parse_mode,
            },
        );
        options.insert(
            String::from("address"),
            Argument {
                description: String::from("Help text"),
                argument_function: Self::parse_address,
            },
        );
        options.insert(
            String::from("port"),
            Argument {
                description: String::from("Help text"),
                argument_function: Self::parse_port,
            },
        );
        options
    }

    fn parse_mode(mode_value: &str) -> Result<Argument, Box<dyn Error>> {
        Err("Not Implemented")?
    }

    fn parse_address(address_value: &str) -> Result<Argument, Box<dyn Error>> {
        Err("Not Implemented")?
    }

    fn parse_port(port_value: &str) -> Result<Argument, Box<dyn Error>> {
        Err("Not Implemented")?
    }

    fn help() {
        println!(
            r#"
Help:
example: program.exe mode=server address=127.0.0.1 port=25786
1. mode = server or client
2. address = ip address of server (client will try to connect server will open here)
3. port = port on which server is operating (client will connect server will listen)
"#
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for Arguments {
        fn eq(&self, other: &Self) -> bool {
            (&self.address, &self.mode, &self.port) == (&other.address, &other.mode, &other.port)
        }
    }

    #[test]
    fn happy_pass() {
        let correct_object = Arguments {
            mode: String::from("client"),
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 42,
        };
        let test_args = vec![
            "mode=client".to_string(),
            "address=127.0.0.1".to_string(),
            "port=42".to_string(),
        ];

        let test_object = Arguments::parse(&test_args);
    }

    // Check this
    // https://stackoverflow.com/questions/43390971/how-to-check-the-exit-code-from-stdprocessexit-in-tests
}
