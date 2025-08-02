mod arp;
mod interfaces;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Interfaces(interfaces::Args),
    ARP(arp::Args),
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Commands::Interfaces(args) => {
            interfaces::list_interfaces(args);
            Ok(())
        }
        Commands::ARP(args) => {
            arp::send_arp(args)
        }
    }
}
