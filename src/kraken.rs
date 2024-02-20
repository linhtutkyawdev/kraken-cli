// kraken.rs
use crate::{
    add::{add_askama, Add},
    execute::Execute,
};
use clap::Subcommand;
use cliclack::{
    confirm, input, intro, log::{error, info, success}, select, set_theme, Theme, ThemeState
};
use colorful::{Color, Colorful};
use console::{style, Style};
use std::{env, fs::OpenOptions, io::Write, process::Command};

pub struct MagentaTheme;

impl Theme for MagentaTheme {
    fn bar_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().magenta(),
            ThemeState::Error(_) => Style::new().red(),
            _ => Style::new().magenta().dim(),
        }
    }

    fn state_symbol_color(&self, _state: &ThemeState) -> Style {
        Style::new().magenta()
    }
}

fn logo() {
    let text = format!(
        "K  K RRRR   AA  K  K EEEE N   N\nK K  R   R A  A K K  E    NN  N\nKK   RRRR  AAAA KK   EEE  N N N\nK K  R R   A  A K K  E    N  NN\nK  K R  RR A  A K  K EEEE N   N"
    );
    println!("{}\n", text.gradient(Color::Magenta));
}

fn add_kraken_toml(language: &str, framework: &str) -> std::io::Result<()> {
    // Check if "Kraken.toml" already exists
    if std::path::Path::new("src/kraken/Kraken.toml").exists() {
        error("Kraken.toml already exists.")?;
    }

    if Command::new("mkdir").arg("src/kraken").status().is_err() {
        error("Failed to run \"mkdir\".")?;
    }

    // If the file doesn't exist, create it and write the content
    let content =
        format!("[kraken]\nlanguage = \"{language}\"\nframework = \"{framework}\"\n\n[features]\n");

    // Use OpenOptions to create the file if it doesn't exist or truncate it if it does
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Truncate the file if it already exists
        .open("src/kraken/Kraken.toml");

    if file.is_err() {
        error("Failed to open the path \"src/kraken/Kraken.toml\"")?;
    }

    if file.unwrap().write_all(content.as_bytes()).is_err() {
        error("Failed writing to path \"src/kraken/Kraken.toml\"")?;
    };
    success("Kraken.toml added successfully.")
}

fn initialize() -> std::io::Result<()> {
    if Command::new("clear").status().is_err() {
        error("Failed to run \"mkdir\".")?;
    }
    // std::process::Command::new("cls").status().unwrap();
    logo();
    set_theme(MagentaTheme);
    intro(style(" kraken ").on_magenta().black())?;
    let language: &str = match select("Pick a project language")
        .item("rs", "Rust", "Performance")
        .item("ts", "Typescript", "DX")
        .item("js", "Javascript", "Oh no!")
        .interact()?
    {
        "rs" => {
            info("🚀 Ahoy, Fearless Rustacean Explorer! 🦀")?;
            "rs"
        }
        _ => {
            info("Wrong answer, just use rust! It's like programming, but for grown-ups.")?;
            "rs"
        }
    };
    let framework: &str = match language {
        "rs" | _ => select("Pick a project language")
            .item("axum", "Axum", "Only Option")
            .interact()?,
    };

    add_kraken_toml(language, framework)
}

fn create() -> std::io::Result<()> {
    if Command::new("clear").status().is_err() {
        error("Failed to run \"clear\".")?;
    }
    // std::process::Command::new("cls").status().unwrap();
    logo();
    set_theme(MagentaTheme);
    intro(style(" kraken ").on_magenta().black())?;
    let name: String = input("Enter your project name.")
        .placeholder("my-kraken-project")
        .default_input("my-kraken-project")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()?;

    if Command::new("clear").status().is_err() {
        error("Failed to run \"clear\".")?;
    }

    if Command::new("cargo")
        .args([
            "shuttle",
            "init",
            "-t",
            "axum",
            &name,
            "--name",
            &name,
            "--create-env",
        ])
        .status()
        .is_err()
    {
        error("Failed to run \"cargo shuttle init\".")?;
    }

    if env::set_current_dir(&name).is_err() {
        error("Failed to change cuttent_dir.")?;
    }

    add_kraken_toml("rs", "axum")?;

    if Command::new("clear").status().is_err() {
        error("Failed to run \"clear\".")?;
    }
    // std::process::Command::new("cls").status().unwrap();
    logo();
    intro(style(" kraken ").on_magenta().black())?;

    if confirm("Do you want to add full html serving feature?")
        .initial_value(true)
        .interact()?
    {
        add_askama()?;
        // add_tailwindcss()?;
        // add_htmx()?;
    }

    println!("Now you can run \"cd {}\" and \"cargo shuttle run\".",name);
    Ok(())
}

#[derive(Subcommand)]
pub enum Kraken {
    /// Create a kraken project
    Create,
    /// Initialize kraken to an existing shuttle project
    Init,
    /// Add super powers
    Add {
        #[command(subcommand)]
        add_commands: Add,
    },
}

impl Execute for Kraken {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Create => {
                create()?;
                Ok(())
            }
            Self::Init => {
                initialize()?;
                Ok(())
            }
            Self::Add { add_commands } => add_commands.execute(),
        }
    }
}
