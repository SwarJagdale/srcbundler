use std::{fs, path::Path, env, io};
use walkdir::WalkDir;
use minifier::js;
use arboard::Clipboard;
use colored::*;

fn main() {
    let src_dir = "src";
    let output_file = "output.txt";
    let args: Vec<String> = env::args().collect();
    let retain_comments = args.contains(&"--keep-comments".to_string());

    if !Path::new(src_dir).exists() {
        eprintln!("Error: src directory not found");
        std::process::exit(1);
    }

    let mut output = String::new();
    let mut total_chars = 0;
    let mut total_files = 0;

    for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx" {
                    if let Ok(content) = fs::read_to_string(path) {
                        let minified = minify_code(&content, retain_comments);
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

    if let Err(e) = fs::write(output_file, &output) {
        eprintln!("Error writing to file: {}", e);
        std::process::exit(1);
    }

    // Print Stats
    println!("\n==========================");
    println!("Processed {} files.", total_files);
    println!("Total characters: {}", total_chars);
    println!("==========================\n");

    
        println!("{}", "[C] Copy full minified content to clipboard or [Q] to quit".cyan());

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "c" {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(output.clone()).unwrap();
            println!("✅ Full minified content copied to clipboard!\n");
        } 
        // }

}

fn minify_code(code: &str, retain_comments: bool) -> String {
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
