use async_std::net::{IpAddr, Ipv4Addr};
use std::collections::HashMap;
use std::process;

#[derive(Debug)]
pub struct ParsedArguments {
    mode: String,
    address: async_std::net::IpAddr,
    port: u16,
}

impl ParsedArguments {
    pub fn parse(args: &Vec<String>) -> Self {
        let mut mode = String::from("");
        let mut address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let mut port = 0;
        let arg_map = Self::separate(args);

        for (key, value) in arg_map.iter() {
            match *key {
                "mode" => {
                    mode = String::from(*value);
                }
                "address" => {
                    address = (*value).parse::<IpAddr>().unwrap();
                }
                "port" => {
                    port = (*value).parse::<u16>().unwrap();
                }
                _ => {
                    Self::help();
                }
            }
        }

        ParsedArguments {
            mode,
            address,
            port,
        }
    }
    fn separate<'until_main>(
        args: &'until_main Vec<String>,
    ) -> HashMap<&'until_main str, &'until_main str> {
        let mut arg_map = HashMap::new();
        for arg in args {
            let split_vector: Vec<&str> = arg.split("=").collect();
            if split_vector.len() != 2 {
                Self::help();
            }
            let result = arg_map.insert(split_vector[0], split_vector[1]);
            match result {
                Some(_inner) => {
                    Self::help();
                }
                None => {}
            }
        }
        arg_map
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
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for ParsedArguments
    {
        fn eq(&self, other: &Self) -> bool {
            (&self.address, &self.mode, &self.port) == (&other.address, &other.mode, &other.port)
        }
    }

    #[test]
    fn happy_pass() {
        let correct_object = ParsedArguments {
            mode: String::from("client"),
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 42,
        };
        let test_args = vec![
            "mode=client".to_string(),
            "address=127.0.0.1".to_string(),
            "port=42".to_string(),
        ];

        let test_object = ParsedArguments::parse(&test_args);
        assert_eq!(correct_object, test_object);
    }

    // Check this
    // https://stackoverflow.com/questions/43390971/how-to-check-the-exit-code-from-stdprocessexit-in-tests
}
