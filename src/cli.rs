use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "forge")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {

    New {
        name: String,
    },

}
