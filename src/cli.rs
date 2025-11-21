use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "yank",
    about = "A simple key-value clipboard manager",
    override_usage = "yank <KEY>\n  or: yank <COMMAND> [ARGS]"
)]
pub struct Cli {
    /// Key to yank (default action)
    pub key: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Store a value under a key
    Put {
        /// The key to store the value under
        key: String,
        /// The value to store
        value: String,
    },

    /// Delete a stored key
    Delete {
        /// The key to delete
        key: String,
    },

    /// List all stored keys
    Ls,
}
