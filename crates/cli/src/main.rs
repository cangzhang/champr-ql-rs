use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Sync,
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "cli", long_about = "sync ChampR builds")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), ()>{
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sync) => {
            println!("sync command");
        }
        _ => {
            println!("nothing");
        }
    };

    Ok(())
}
