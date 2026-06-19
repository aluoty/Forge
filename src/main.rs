mod cli;
mod generator;
mod templates;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::New { name, lang, template } => {
            let lang_id = match lang {
                Some(id) => id,
                None => templates::interactive_select_lang()?,
            };

            let template_id = match template {
                Some(id) => Some(id),
                None => templates::interactive_select_template(&lang_id)?,
            };

            generator::create_project(&name, &lang_id, template_id.as_deref())?;
            let tpl_msg = template_id
                .map(|t| format!(" with template '{t}'"))
                .unwrap_or_default();
            println!("Created project '{name}' ({lang_id}{tpl_msg})");
        }
        cli::Commands::List => {
            println!("Languages:");
            for l in templates::LANGUAGES {
                println!("  {:<14} {} — {}", l.id, l.label, l.desc);
            }
            let c_tpls = templates::templates_for_lang("c");
            if !c_tpls.is_empty() {
                println!("\nTemplates for C:");
                for t in &c_tpls {
                    println!("  forge new <name> --lang c --template {:<9}  {} — {}", t.id, t.label, t.desc);
                }
            }
        }
        cli::Commands::Add { name, packages } => {
            generator::add_packages(&name, &packages)?;
        }
        cli::Commands::Doctor => {
            templates::doctor();
        }
    }

    Ok(())
}
