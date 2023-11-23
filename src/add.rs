// add.rs
use crate::{execute::Execute, kraken::MagentaTheme};
use clap::Subcommand;
use cliclack::{confirm, intro, log, outro, set_theme, spinner};
use console::style;
use std::fs::{self, create_dir_all, read_to_string, File, OpenOptions};
use std::io::prelude::*;
use std::process::{Command, Stdio};
use toml::{self, Value};

#[derive(Subcommand)]
pub enum Add {
    /// Add askama templating engine for html
    Askama,
    /// Add everyone's favorite - tailwindcss
    Tailwindcss,
    /// The real solution : HTMX
    Htmx,
    /// Create a new askama page
    Page,
}

impl Execute for Add {
    fn execute(&self) -> anyhow::Result<()> {
        set_theme(MagentaTheme);
        intro(style(" kraken ").on_magenta().black())?;
        match self {
            Self::Askama => {
                check_feature_exists("askama").unwrap();
                let mut spinner = spinner();
                spinner.start("Adding crates!");
                add_dependencies();
                spinner.stop("Crates have arrived!");
                match create_html_base_file() {
                    Ok(()) => log::info("Added base.html or layout!")?,
                    Err(_err) => log::error("Error Adding base.html!")?,
                }
                if confirm("Do you want to create a page?")
                    .initial_value(true)
                    .interact()?
                {
                    log::info("create page. 🎉")?;
                }
                outro("Askama added successfully. 🎉")?;
                Ok(())
            }
            Self::Tailwindcss => {
                let mut spinner = spinner();
                spinner.start("Adding crates!");
                // check_if_base.html_exists
                if std::path::Path::new("./templates/base.html").exists() {
                    // edit base.html and add some link tag
                } else {
                    // error
                }
                spinner.stop("Crates have arrived! 🎉");
                println!("Tailwindcss added!");
                Ok(())
            }
            Self::Htmx => {
                // check_if_base.html_exists
                if std::path::Path::new("./templates/base.html").exists() {
                    // edit base.html and add some link tag
                    match add_htmx_script() {
                        Ok(()) => println!("base.html edited successfully."),
                        Err(err) => eprintln!("Error editing base.html: {}", err),
                    };
                } else {
                    // error
                }
                println!("Htmx added!");
                Ok(())
            }
            Self::Page => {
                // check_if_base.html_exists
                if std::path::Path::new("./templates/base.html").exists() {
                    // edit base.html and add some link tag
                    match add_htmx_script() {
                        Ok(()) => println!("base.html edited successfully."),
                        Err(err) => eprintln!("Error editing base.html: {}", err),
                    };
                } else {
                    // error
                }
                println!("Htmx added!");
                Ok(())
            }
        }
    }
}

fn add_dependencies() {
    // Create a new Command
    let mut cmd = Command::new("cargo")
        .args(["add", "askama", "-F", "askama/with-axum", "askama_axum"])
        .stdout(Stdio::null()) // Redirect stdout to null sink
        .stderr(Stdio::null()) // Redirect stderr to null sink
        .spawn()
        .expect("Failed to start cargo command");

    // Wait for the child process to finish
    let status = cmd.wait().expect("Failed to wait for cargo command");
    if !status.success() {
        eprintln!("cargo command failed with exit code: {}", status);
    }
}

fn create_html_base_file() -> Result<(), std::io::Error> {
    // Create the templates directory if it doesn't exist
    let templates_path = format!("./templates");
    create_dir_all(&templates_path)?;

    // Create base.html
    let base_html_path = format!("{}/base.html", templates_path);
    let mut base_html_file = File::create(&base_html_path)?;
    base_html_file.write_all(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{% block title %}{% endblock %}</title>
    {% block head %}{% endblock %}
  </head>
  <body>
    {% block content %}{% endblock %}
  </body>
</html>
    "#
        .as_bytes(),
    )?;

    Ok(())
}

fn add_htmx_script() -> Result<(), std::io::Error> {
    // Read existing content of base.html
    let mut base_html_content = read_to_string("./templates/base.html")?;
    if base_html_content
        .find("https://unpkg.com/htmx.org@")
        .is_none()
    {
        // Find the index of the closing </head> tag
        if let Some(head_close_index) = base_html_content.find("</head>") {
            // Add link tags just before the </head> tag
            base_html_content.insert_str(
                head_close_index,
                r#"
    <!-- cdn deez nuts for htmx -->
    <script
      src="https://unpkg.com/htmx.org@1.9.8"
      integrity="sha384-rgjA7mptc2ETQqXoYC3/zJvkU7K/aP44Y+z7xQuJiVnB/422P/Ak+F/AqFR7E4Wr"
      crossorigin="anonymous"
    ></script>
    "#,
            );

            // Open base.html in write mode and truncate its content
            let mut base_html_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("./templates/base.html")?;

            // Write the modified content back to base.html
            base_html_file.write_all(base_html_content.as_bytes())?;

            Ok(())
        } else {
            eprintln!("Failed to find </head> tag in base.html");
            Ok(()) // or Err(...) depending on your error handling strategy
        }
    } else {
        eprintln!("Htmx script already exists!");
        Ok(()) // or Err(...) depending on your error handling strategy
    }
}

fn check_feature_exists(feature_name: &str) -> Result<(), std::io::Error> {
    if fs::metadata("Kraken.toml").is_ok() {
        let toml_content = fs::read_to_string("Kraken.toml")?;

        // Parse the TOML content
        let parsed_toml: Result<Value, toml::de::Error> = toml::from_str(&toml_content);

        match parsed_toml {
            Ok(parsed_toml) => {
                // Check if [features] section exists
                if let Some(features) = parsed_toml.get("features") {
                    // Check if the specific feature exists
                    if let Some(_) = features.get(feature_name) {
                        println!("The [features.{}] exists in Kraken.toml", feature_name);
                    } else {
                        println!(
                            "The [features.{}] does not exist in Kraken.toml",
                            feature_name
                        );
                    }
                } else {
                    println!("The [features] section does not exist in Kraken.toml");
                }
            }
            Err(err) => {
                eprintln!("Error parsing Kraken.toml: {}", err);
            }
        }
    } else {
        println!("Kraken.toml does not exist");
    }

    Ok(())
}
