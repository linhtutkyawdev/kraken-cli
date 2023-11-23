// kraken.rs
use crate::{add::Add, execute::Execute};
use clap::Subcommand;
use cliclack::{intro, log, outro, select, set_theme, Theme, ThemeState};
use colorful::{Color, Colorful};
use console::{style, Style};
use std::{fs::OpenOptions, io::Write, process::Command};

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

fn add_kraken_toml(language: &str, framework: &str) -> Result<(), std::io::Error> {
    // Check if "Kraken.toml" already exists
    if std::path::Path::new("Kraken.toml").exists() {
        // If the file exists, return an error or handle it as needed
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Kraken.toml already exists",
        ));
    }

    // If the file doesn't exist, create it and write the content
    let content =
        format!("[kraken]\nlanguage = \"{language}\"\nframework = \"{framework}\"\n\n[features]\n");

    // Use OpenOptions to create the file if it doesn't exist or truncate it if it does
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Truncate the file if it already exists
        .open("Kraken.toml")?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

fn ask_info() -> std::io::Result<()> {
    Command::new("clear").status().unwrap();
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
            log::info(format!("🚀 Ahoy, Fearless Rustacean Explorer! 🦀"))?;
            "rs"
        }
        _ => {
            log::info(format!(
                "Wrong answer, just use rust! It's like programming, but for grown-ups."
            ))?;
            "rs"
        }
    };
    let framework: &str = match language {
        "rs" | _ => select("Pick a project language")
            .item("axum", "Axum", "Only Option")
            .interact()?,
    };

    add_kraken_toml(language, framework)?;
    outro("Kraken Initialized")?;

    Ok(())
}

#[derive(Subcommand)]
pub enum Kraken {
    /// Initialize kraken
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
            Self::Init => {
                ask_info()?;
                Ok(())
            }
            Self::Add { add_commands } => add_commands.execute(),
        }
    }
}
