// add.rs
use crate::{execute::Execute, kraken::MagentaTheme};
use clap::Subcommand;
use cliclack::log::{error, info};
use cliclack::{confirm, input, intro, outro, set_theme, spinner};
use console::style;
use proc_macro2::{Ident, Span};
use quote::quote;
use regex::Regex;
use std::fs::{self, create_dir_all, read_to_string, File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::process::{Command, Stdio};
use toml_edit::Document;

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

        // Check if "Kraken.toml" exists
        if !std::path::Path::new("src/kraken/Kraken.toml").exists() {
            error("Kraken not initialized.")?;
            return Ok(());
        }

        match self {
            Self::Askama => {
                add_askama()?;
                Ok(())
            }
            Self::Tailwindcss => {
                add_tailwindcss()?;
                Ok(())
            }
            Self::Htmx => {
                add_htmx()?;
                Ok(())
            }
            Self::Page => {
                add_page()?;
                Ok(())
            }
        }
    }
}

pub fn add_askama() -> std::io::Result<()> {
    match check_feature("askama") {
        Ok(()) => {
            let mut spinner = spinner();
            spinner.start("Adding crates!");
            add_dependencies();
            spinner.stop("Crates have arrived!");
            match create_html_base_file() {
                Ok(()) => info("Added base.html or layout!")?,
                Err(_err) => error("Error Adding base.html!")?,
            }
            add_feature("askama").unwrap();
            info("Askama added successfully. 🎉")?;
            if confirm("Do you want to create a page?")
                .initial_value(true)
                .interact()?
            {
                add_page()?;
            }
        }
        Err(err) => {
            error(err)?;
            info("Failed to add askama!!!")?;
        }
    };
    Ok(())
}

pub fn add_tailwindcss() -> std::io::Result<()> {
    generate_tailwindcss_mod_rs()?;
    add_module_to_mod_rs("tailwindcss")?;
    add_module_to_main_rs("tailwindcss")?;
    call_module_fn_in_main_rs("tailwindcss", Some("styles/tailwind.css"))?;

    if !std::path::Path::new("./tailwind.config.js").exists() {
        if Command::new("tailwindcss").arg("init").status().is_err() {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "tailwindcss command faied!",
            ));
        }
    }

    edit_tailwind_config()?;

    if !std::path::Path::new("./styles/styles.js").exists() {
        match create_tailwind_base_styles() {
            Ok(()) => println!("created styles/styles.css."),
            Err(err) => eprintln!("Error creating styles/styles.css: {}", err),
        }
    }

    // tailwindcss -i styles/styles.css -o styles/tailwind.css
    if Command::new("tailwindcss")
        .args(["-i", "styles/styles.css", "-o", "styles/tailwind.css"])
        .status()
        .is_err()
    {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "tailwindcss command faied!",
        ));
    }

    if std::path::Path::new("./templates/base.html").exists() {
        // edit base.html and add some link tag
        match add_tag_to_head(
            r#"
<!-- tailwind deez css -->
<link rel="stylesheet" href="/styles/tailwind.css" />
"#,
            "/styles/tailwind.css",
        ) {
            Ok(()) => println!("base.html edited successfully."),
            Err(err) => eprintln!("Error editing base.html: {}", err),
        };
        println!("Tailwindcss added!");
    } else {
        println!("No base.html!");
    }
    Ok(())
}

fn edit_tailwind_config() -> std::io::Result<()> {
    let file_path = "./tailwind.config.js";
    if !std::path::Path::new(&file_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "tailwind.config.js not found!",
        ));
    }

    let file_content: Vec<String> = fs::read_to_string(&file_path)?
        .lines()
        .map(|line| {
            if line.contains("content: ") {
                r#"  content: ["./templates/**/*.html"],"#.to_string()
            } else {
                line.to_string()
            }
        })
        .collect();

    fs::write(&file_path, file_content.join("\n"))
}

pub fn add_dependencies() {
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

pub fn add_htmx() -> std::io::Result<()> {
    // check_if_base.html_exists
    if std::path::Path::new("./templates/base.html").exists() {
        // edit base.html and add some link tag
        match add_tag_to_head(
            r#"
        <!-- cdn deez nuts for htmx -->
        <script src="https://unpkg.com/htmx.org@1.9.10"></script>
    "#,
            "https://unpkg.com/htmx.org@",
        ) {
            Ok(()) => println!("base.html edited successfully."),
            Err(err) => eprintln!("Error editing base.html: {}", err),
        };
        println!("Htmx added!");
    } else {
        println!("No base.html!");
    }
    Ok(())
}

pub fn create_html_base_file() -> Result<(), std::io::Error> {
    // Create the templates directory if it doesn't exist
    let templates_path = format!("templates");
    create_dir_all(&templates_path)?;

    // Create base.html
    let base_html_path = format!("{templates_path}/base.html");
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

pub fn create_tailwind_base_styles() -> Result<(), std::io::Error> {
    // Create the styles directory if it doesn't exist
    let styles_path = format!("styles");
    create_dir_all(&styles_path)?;

    // Create base.html
    let base_styles_path = format!("{styles_path}/styles.css");
    let mut base_styles_file = File::create(&base_styles_path)?;
    base_styles_file.write_all(
        r#"@tailwind base;
@tailwind components;
@tailwind utilities;
"#
        .as_bytes(),
    )?;

    Ok(())
}

pub fn add_tag_to_head(tag: &str, identity: &str) -> Result<(), std::io::Error> {
    // Read existing content of base.html
    let mut base_html_content = read_to_string("./templates/base.html")?;
    if base_html_content.find(identity).is_none() {
        // Find the index of the closing </head> tag
        if let Some(head_close_index) = base_html_content.find("</head>") {
            // Add link tags just before the </head> tag
            base_html_content.insert_str(head_close_index, tag);

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

pub fn kraken_toml_exists() -> bool {
    fs::metadata("src/kraken/Kraken.toml").is_ok()
}

pub fn add_feature(key: &str) -> Result<(), std::io::Error> {
    if kraken_toml_exists() {
        let toml_content = fs::read_to_string("src/kraken/Kraken.toml")?;
        let mut doc = toml_content.parse::<Document>().expect("invalid doc");
        doc["features"][key] = toml_edit::value(true);
        fs::write("src/kraken/Kraken.toml", doc.to_string())?;
    }
    Ok(())
}

pub fn check_feature(key: &str) -> Result<(), std::io::Error> {
    if kraken_toml_exists() {
        if fs::read_to_string("src/kraken/Kraken.toml")?
            .parse::<Document>()
            .expect("invalid doc")["features"]
            .get(key)
            .is_some()
        {
            error("Features already exists!")?;
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Features already exists!",
            ));
        }
    }
    Ok(())
}

pub fn generate_page_template(page_name: &str, ai_generated_htmx: &str) -> std::io::Result<()> {
    // Create the templates directory if it doesn't exist
    create_dir_all(&"templates")?;
    // Create page.html
    let mut base_html_file = File::create(format!("templates/{page_name}.html"))?;
    base_html_file.write_all(
        format!(
            r#"
<!-- prettier-ignore -->
{{% extends "base.html" %}}
{{% block title %}}{{{{title}}}}{{% endblock %}}
{{% block content %}}
{ai_generated_htmx}
{{% endblock %}}"#
        )
        .as_bytes(),
    )?;

    Ok(())
}

pub fn generate_page_mod_rs(page_name: &str, page_title: &str) -> std::io::Result<()> {
    let page_file = format!("{page_name}.html");
    let code = quote! {
        use askama::Template;
        use askama_axum::IntoResponse;

        #[derive(Template)]
        #[template(path = #page_file)]
        struct TheTemplate<'a> {
            title: &'a str,
        }

        pub async fn main() -> impl IntoResponse {
            TheTemplate {
                title: #page_title,
            }
        }
    };

    // Change the path to your desired location for the mod.rs file
    let file_path = format!("src/kraken/{page_name}.rs");

    let mut file = File::create(file_path)?;

    // Write the generated code to the file
    file.write_all(code.to_string().as_bytes())?;

    // Run rustfmt on the prettify file
    if Command::new("rustfmt")
        .arg(format!("src/kraken/{page_name}.rs"))
        .arg("--edition")
        .arg("2021")
        .status()
        .is_err()
    {
        error("Failed to run rustfmt")?;

        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Failed to run rustfmt",
        ));
    }

    Ok(())
}

pub fn add_kraken_to_main_rs() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("src/main.rs")?;

    // Read the existing content
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Check if the module already exists
    if !content.contains(&format!("mod kraken;")) {
        // Append the new module
        let new_module_code = quote! {
            mod kraken;
        };

        let syntax_tree = syn::parse_file(&new_module_code.to_string()).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        content = formatted + &content;

        // Seek to the beginning of the file and write the modified content
        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

pub fn add_module_to_mod_rs(module_name: &str) -> std::io::Result<()> {
    let module_name = Ident::new(module_name, Span::call_site());

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("src/kraken/mod.rs")?;

    // Read the existing content
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Check if the module already exists
    if content.contains(&format!("pub mod {};", module_name)) {
        error("Module already exists.")?;
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Module already exists.",
        ));
    } else {
        // Append the new module
        let new_module_code = quote! {
            pub mod #module_name;
        };

        let syntax_tree = syn::parse_file(&new_module_code.to_string()).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        content = formatted + &content;

        // Seek to the beginning of the file and write the modified content
        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

pub fn call_module_fn_in_main_rs(module_name: &str, src: Option<&str>) -> std::io::Result<()> {
    let route = match src {
        Some(r) => get_route(r),
        None => get_route(module_name),
    };

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("src/main.rs")?;

    // Read the existing content
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    content = content.replace(
        r#"
async fn hello_world() -> &'static str {
    "Hello, world!"
}
"#,
        "",
    );

    let pattern = format!(r#".route\(\"/{}\", get\((\w+)\)"#, route);
    let replacement = format!("\n.route(\"/{}\", get({}::main)", route, module_name);
    let regex = Regex::new(&pattern).unwrap();
    content = regex.replace(&content, &replacement).to_string();

    if !content.contains(&replacement) {
        let old = "Router::new()";
        let new = old.to_string() + &replacement + ")";
        content = content.replace(old, &new);
    }
    // Seek to the beginning of the file and write the modified content
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn get_route(module_name: &str) -> &str {
    if module_name == "index" {
        return "";
    }
    return module_name;
}

pub fn add_module_to_main_rs(module_name: &str) -> std::io::Result<()> {
    let module_name = Ident::new(module_name, Span::call_site());

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("src/main.rs")?;

    // Read the existing content
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let pattern = format!(r"kraken::\{{.*{}.*\}}", module_name);
    let regex = Regex::new(&pattern).unwrap();

    // Check if the line matches the pattern
    if regex.is_match(&content) {
        error("Module already used.")?;

        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Module already used.",
        ));
    } else {
        // Append the new module
        let new_module_code = quote! {
            use kraken::#module_name;
        };

        let syntax_tree = syn::parse_file(&new_module_code.to_string()).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        content = formatted + &content;

        // Seek to the beginning of the file and write the modified content
        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}

/// Capitalizes the first character in s.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn add_page() -> std::io::Result<()> {
    let page_name: String = input("Page name").default_input("index").interact()?;
    let page_title: String = input("Page title")
        .default_input(&capitalize(&page_name))
        .interact()?;

    if generate_page_template(
        &page_name,
        r#"<section class="bg-indigo-400 text-white font-black text-6xl">Hello</section>"#,
    )
    .is_err()
    {
        outro("An error occured!")?;
        return Ok(());
    }

    if generate_page_mod_rs(&page_name, &page_title).is_err() {
        outro("An error occured!")?;
        return Ok(());
    }
    if add_module_to_mod_rs(&page_name).is_err() {
        outro("An error occured!")?;
        return Ok(());
    }
    if add_module_to_main_rs(&page_name).is_err() {
        outro("An error occured!")?;
        return Ok(());
    }
    if add_kraken_to_main_rs().is_err() {
        outro("An error occured!")?;
        return Ok(());
    }
    if call_module_fn_in_main_rs(&page_name, None).is_err() {
        outro("An error occured!")?;
        return Ok(());
    }

    outro(format!("Successfully created: {page_name} page. 🎉"))
}

pub fn generate_tailwindcss_mod_rs() -> std::io::Result<()> {
    let code = quote! {
        use askama_axum::{IntoResponse, Response};
        use axum::http::StatusCode;

        pub async fn main() -> impl IntoResponse {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/css")
                .body(include_str!("../../styles/tailwind.css").to_owned())
                .unwrap()
        }
    };

    let mut file = File::create("src/kraken/tailwindcss.rs")?;

    // Write the generated code to the file
    file.write_all(code.to_string().as_bytes())?;

    // Run rustfmt on the prettify file
    Command::new("rustfmt")
        .arg("src/kraken/tailwindcss.rs")
        .arg("--edition")
        .arg("2021")
        .status()
        .expect("Failed to run rustfmt");

    Ok(())
}
