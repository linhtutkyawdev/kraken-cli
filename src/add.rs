// add.rs
use clap::Subcommand;

use crate::execute::Execute;

#[derive(Subcommand)]
pub enum Add {
    /// Add askama templating engine for html (or) htmx
    Askama,
    /// Add everyone's favorite - tailwindcss
    Tailwindcss,
}

impl Execute for Add {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Askama => {
                println!("Askama added!");
                Ok(())
            }
            Self::Tailwindcss => {
                println!("Tailwindcss added!");
                Ok(())
            }
        }
    }
}
