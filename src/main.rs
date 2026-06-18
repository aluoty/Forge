mod cli;
mod generator;
mod templates;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::New { name, lang } => {
            let template_id = match lang {
                Some(id) => id,
                None => templates::interactive_select()?,
            };
            generator::create_project(&name, &template_id)?;
            println!("Created project '{name}' with template '{template_id}'");
        }
        cli::Commands::List => {
            templates::list_templates();
        }
        cli::Commands::Add { name, packages } => {
            generator::add_packages(&name, &packages)?;
        }
    }

    Ok(())
}
