use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "forge", about = "A project scaffold generator for Nix")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: String,
        /// Language (omit for interactive selection)
        #[arg(long, short)]
        lang: Option<String>,
        /// Language template (e.g. raylib, ncurses, sdl2)
        #[arg(long, short)]
        template: Option<String>,
    },
    /// List available languages and templates
    List,
    /// Add Nix packages to an existing project's flake.nix
    Add {
        /// Project directory name
        name: String,
        /// Nix packages to add
        packages: Vec<String>,
    },
    /// Check system setup and list available languages / templates
    Doctor,
}
