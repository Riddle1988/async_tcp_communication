use futures::select;
use futures::FutureExt;

use async_std::{
    io::{stdin, BufReader},
    net::{TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};

#[path = "command_line_parse.rs"] mod command_line_parse;
use crate::command_line_parse::Arguments;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn main(args: &Vec<Arguments>) -> Result<()> {
    let option_address = args
    .iter()
    .filter_map(|f| {
        if let Arguments::Address(ref value) = *f {
            Some(value)
        } else {
            None
        }
    })
    .next()
    .map(ToOwned::to_owned);
    let address = match option_address {
        Some(ip) => ip,
        None => Err("Wrong address")?
    };

    let option_port = args
    .iter()
    .filter_map(|f| {
        if let Arguments::Port(ref value) = *f {
            Some(value)
        } else {
            None
        }
    })
    .next()
    .map(ToOwned::to_owned);
    let port = match option_port {
        Some(number) => number,
        None => Err("Wrong port")?
    };

    let mut full_address = address.to_string();
    full_address.push_str(":");
    full_address.push_str(port.to_string().as_ref());
    task::block_on(try_main(full_address))
}

async fn try_main(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = (&stream, &stream);
    let reader = BufReader::new(reader);
    let mut lines_from_server = futures::StreamExt::fuse(reader.lines());

    let stdin = BufReader::new(stdin());
    let mut lines_from_stdin = futures::StreamExt::fuse(stdin.lines());
    loop {
        select! {
            line = lines_from_server.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    println!("{}", line);
                },
                None => break,
            },
            line = lines_from_stdin.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                None => break,
            }
        }
    }
    Ok(())
}