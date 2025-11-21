use clap::Parser;
use yank::{
    cli::{Cli, Commands},
    handler::{Handler, Result},
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let mut handler = Handler::new()?;
    handler.load_data()?;

    match cli.command {
        Some(Commands::Ls) => handler.list_keys()?,
        Some(Commands::Put { key, value }) => handler.set_value(&key, value)?,
        Some(Commands::Delete { key }) => handler.delete_value(&key)?,
        None => {
            if let Some(key) = cli.key {
                handler.yank_value(&key)?;
            } else {
                eprintln!("No key provided");
            }
        }
    }

    Ok(())
}
