use std::io::Error;

use clap::{Args, Parser, Subcommand};

mod docker;
mod server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the dockio server
    Start,
    /// Generate generic diagram of current server
    Gen,
}

#[derive(Args)]
struct Start {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // console_subscriber::init();
    let _ = env_logger::try_init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start) | None => {
            server::start_server().await?;
        },
        Some(Commands::Gen) => {
            let f_containers = docker::get_containers();

            let str = serde_json::to_string(&f_containers).unwrap();

            println!("{}", str);
        }
    }

    Ok(())
}
