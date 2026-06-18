mod cli;
mod generator;

use clap::Parser;
use cli::{Args, Commands};


fn main() -> anyhow::Result<()> {

    let args = Args::parse();


    match args.command {

        Commands::New { name } => {
            generator::create_project(&name)?;

            println!("Created {}", name);
        }

    }


    Ok(())
}
