use cliclack::{input, intro, log, outro, set_theme, Theme, ThemeState};
use colorful::Colorful;
use console::{style, Style};
struct MagentaTheme;

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

    fn info_symbol(&self) -> String {
        "⚙".into()
    }
}

fn main() -> std::io::Result<()> {
    std::process::Command::new("clear").status().unwrap();
    // std::process::Command::new("cls").status().unwrap();
    let text = format!(
        "K  K RRRR   AA  K  K EEEE N   N\nK K  R   R A  A K K  E    NN  N\nKK   RRRR  AAAA KK   EEE  N N N\nK K  R R   A  A K K  E    N  NN\nK  K R  RR A  A K  K EEEE N   N"
    );
    text.rainbow();

    print!("\n\n\n\n\n\n");

    set_theme(MagentaTheme);
    intro(style(" kraken ").on_magenta().black())?;
    let path: String = input("Where should we create your project?")
        .placeholder("./right-here")
        .interact()?;

    log::info(format!("Project path: {path}"))?;

    outro("Done")?;

    Ok(())
}
