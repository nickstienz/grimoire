use clap::Parser;
use cli::Commands;

mod cli;

fn main() {
    let args = cli::Args::parse();

    match args.command {
        Commands::New(new_args) => {
            println!("Creating project: {}", new_args.name);
        }
    }
}
