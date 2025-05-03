use std::{fs, path::Path, io::{self, Write}};
use clap::{Arg, Command, ArgAction};
use walkdir::WalkDir;
use minifier::js;
use minifier::css;
use arboard::Clipboard;
use colored::*;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use ctrlc;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    extensions: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            extensions: vec!["js".to_string(), "ts".to_string(), "jsx".to_string(), 
                           "tsx".to_string(), "css".to_string()]
        }
    }
}

fn get_config() -> Config {
    let proj_dirs = ProjectDirs::from("com", "srcbundler", "srcbundler")
        .expect("Failed to determine config directory");
    
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("config.json");

    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }

    if !config_path.exists() {
        let default_config = Config::default();
        let config_json = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default config");
        fs::write(&config_path, config_json).expect("Failed to write default config");
        default_config
    } else {
        let config_content = fs::read_to_string(&config_path)
            .expect("Failed to read config file");
        serde_json::from_str(&config_content)
            .expect("Failed to parse config file")
    }
}

fn main() {
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C, exiting...");
        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    let matches = Command::new("SrcBundler")
        .version("1.0")
        .author("Your Name")
        .about("Minifies and combines web development source files")
        .arg(Arg::new("keep-comments")
            .long("keep-comments")
            .short('k')
            .help("Retains comments in minified files")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("save")
            .long("save")
            .short('s')
            .value_name("FILE")
            .help("Saves output to a specified file"))
        .arg(Arg::new("dir")
            .long("dir")
            .short('d')
            .value_name("DIR")
            .help("Source directory to process (default: src)")
            .default_value("src"))
        .arg(Arg::new("ext")
            .long("ext")
            .short('e')
            .value_name("EXT")
            .help("Comma-separated list of file extensions to process (overrides config file)")
            .value_parser(clap::value_parser!(String)))
        .get_matches();

    let src_dir = matches.get_one::<String>("dir").unwrap();
    let retain_comments = matches.get_flag("keep-comments");
    let save_to_file = matches.get_one::<String>("save").map(|s| s.as_str());

    
    let extensions = if let Some(ext_arg) = matches.get_one::<String>("ext") {
        ext_arg.split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    } else {
        get_config().extensions
    };

    if !Path::new(src_dir).exists() {
        eprintln!("Error: source directory '{}' not found", src_dir);
        std::process::exit(1);
    }

    let mut output = String::new();
    let mut total_chars = 0;
    let mut total_files = 0;

    for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if extensions.contains(&ext.to_string_lossy().to_string()) {
                    if let Ok(content) = fs::read_to_string(path) {
                        let minified = minify_code(&content, retain_comments, ext.to_str().unwrap());
                        let relative_path = path.strip_prefix(src_dir).unwrap().to_string_lossy();
                        let context_length = minified.chars().count();

                        output.push_str(&format!("``` {relative_path}\n{minified}\n```\n"));
                        total_chars += context_length;
                        total_files += 1;
                    }
                }
            }
        }
    }

    println!("\n==========================");
    println!("Processed {} files.", total_files);
    println!("Total characters: {}", total_chars);
    println!("==========================\n");

    if let Some(filename) = save_to_file {
        if let Err(e) = fs::write(filename, &output) {
            eprintln!("Error writing to file: {}", e);
            std::process::exit(1);
        }
        println!("✅ Output saved to {}", filename);
    }

    loop {
        print!("{}\n> ", "[C] Copy full minified content to clipboard or [Q] to quit".cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_lowercase();
                match input.as_str() {
                    "c" => {
                        match Clipboard::new() {
                            Ok(mut clipboard) => {
                                match clipboard.set_text(output.clone()) {
                                    Ok(_) => println!("✅ Full minified content copied to clipboard!\n"),
                                    Err(e) => eprintln!("Error copying to clipboard: {}", e)
                                }
                            },
                            Err(e) => eprintln!("Error accessing clipboard: {}", e)
                        }
                    },
                    "q" => {
                        println!("Goodbye!");
                        break;
                    },
                    _ => println!("Invalid input. Please enter 'C' to copy or 'Q' to quit.")
                }
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn minify_code(code: &str, retain_comments: bool, file_type: &str) -> String {
    match file_type {
        "css" => match css::minify(code) {
            Ok(minified) => minified.to_string(),
            Err(_) => code.to_string(),
        },
        _ => {
            if retain_comments {
                let mut result = String::new();
                let mut in_comment = false;

                for line in code.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("//") || in_comment {
                        result.push_str(line);
                        result.push('\n');
                    } else {
                        let minified_line = trimmed;
                        if !minified_line.is_empty() {
                            result.push_str(minified_line);
                            result.push('\n');
                        }
                    }
                    if line.contains("/*") {
                        in_comment = true;
                    }
                    if line.contains("*/") {
                        in_comment = false;
                    }
                }
                result
            } else {
                js::minify(code).to_string()
            }
        }
    }
}
