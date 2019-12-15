/*!
# tcp-warp-cli description

## Features

## Usage

```bash
cargo install tcp-warp-cli
```

*/
use env_logger::Builder as LoggerBuilder;
use std::error::Error;
use structopt::StructOpt;
use tcp_warp::{TcpWarpClient, TcpWarpServer};

mod cli;

use cli::{Cli, Command::*};

const DEFAULT_CLIENT_BIND: &str = "0.0.0.0";
const DEFAULT_CLIENT_SERVER: &str = "127.0.0.1:18000";
const DEFAULT_SERVER_LISTEN: &str = DEFAULT_CLIENT_SERVER;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();

    cli.verbose
        .log_level()
        .map(|x| {
            LoggerBuilder::new()
                .filter(None, x.to_level_filter())
                .try_init()
        })
        .transpose()?;

    match cli.command {
        Client { bind, server, map } => {
            eprintln!("{:?}", map);
            TcpWarpClient::new(
                bind.unwrap_or_else(|| DEFAULT_CLIENT_BIND.into()).parse()?,
                server
                    .unwrap_or_else(|| DEFAULT_CLIENT_SERVER.into())
                    .parse()?,
                map,
            )
            .connect()
            .await
        }
        Server { listen } => {
            TcpWarpServer::new(
                listen
                    .unwrap_or_else(|| DEFAULT_SERVER_LISTEN.into())
                    .parse()?,
            )
            .listen()
            .await
        }
    }
}