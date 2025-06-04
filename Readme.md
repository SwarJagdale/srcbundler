# SrcBundler

## Overview
SrcBundler is a command-line tool written in Rust that recursively scans source directories, extracts and minifies web development files, and outputs the concatenated result in a structured markdown format. It supports configurable file extensions and provides flexible directory processing.

## Features
- **Configurable File Extensions**: Default support for JavaScript (`.js`), TypeScript (`.ts`), JSX (`.jsx`), TSX (`.tsx`), and CSS (`.css`) files
- **Custom Directory Processing**: Specify any source directory (default: `src/`)
- **Flexible Extension Filtering**: Override default extensions via command line or process all files with `*`
- **Configuration Management**: Automatic config file creation and management in user's config directory
- **Comment Retention**: Option to retain comments using `--keep-comments`
- **Statistics Display**: Shows file processing statistics (total files, character count)
- **Clipboard Integration**: Copy the final minified content to clipboard
- **File Output**: Save the output to a file using `--save <filename>`
- **Graceful Exit**: Ctrl+C handler for clean termination
- **Cross-platform**: Works on Windows, macOS, and Linux

## Installation
Ensure you have Rust installed on your system. Then, clone this repository and build the project:

```sh
git clone <repository-url>
cd srcbundler
cargo build --release
```

The executable will be available at `target/release/srcbundler` (or `target/release/srcbundler.exe` on Windows).

## Usage
Navigate to the project directory and run:

```sh
./target/release/srcbundler
```

Or on Windows:
```cmd
.\target\release\srcbundler.exe
```

### Command Line Options
- `--dir, -d <DIR>`: Source directory to process (default: `src`)
- `--ext, -e <EXT>`: Comma-separated list of file extensions to process (overrides config file)
- `--keep-comments, -k`: Retains comments in the minified output
- `--save, -s <FILE>`: Saves the output to a specified file
- `--help`: Display help information
- `--version`: Display version information

### Configuration
SrcBundler automatically creates a configuration file on first run at:
- **Windows**: `%APPDATA%\srcbundler\config.json`
- **macOS**: `~/Library/Application Support/com.srcbundler.srcbundler/config.json`
- **Linux**: `~/.config/srcbundler/config.json`

The default configuration includes:
```json
{
  "extensions": ["js", "ts", "jsx", "tsx", "css"]
}
```

You can modify this file to change default file extensions.

### Example Commands

Basic usage (processes `src/` directory with default extensions):
```sh
./srcbundler
```

Process a different directory:
```sh
./srcbundler --dir assets
```

Process specific file types:
```sh
./srcbundler --ext js,ts,css
```

Process all files in directory:
```sh
./srcbundler --ext "*"
```

Minify while retaining comments:
```sh
./srcbundler --keep-comments
```

Save minified output to a file:
```sh
./srcbundler --save output.txt
```

Combine multiple options:
```sh
./srcbundler --dir frontend --ext js,jsx,css --keep-comments --save bundle.txt
```

### Interactive Mode
After processing, SrcBundler enters an interactive mode where you can:
- Press `C` to copy the full minified content to clipboard
- Press `Q` to quit the application

## Output Format
SrcBundler outputs the processed files in markdown code block format:

```
\``` relative/path/to/file.js
minified content here
\```

\``` relative/path/to/another/file.css
minified css content here
\```
```

After processing, it displays summary statistics:
```
==========================
Processed 15 files.
Total characters: 12543
==========================
```

## Dependencies
- **walkdir**: Recursive directory traversal
- **minifier**: JavaScript and CSS minification
- **arboard**: Clipboard functionality
- **colored**: Terminal output coloring
- **clap**: Command-line argument parsing
- **serde & serde_json**: Configuration file handling
- **directories**: Cross-platform config directory detection
- **ctrlc**: Signal handling for graceful shutdown

## Notes
- **File Processing**: The tool intelligently handles different file types with appropriate minification strategies
- **Error Handling**: Gracefully handles missing directories, file read errors, and clipboard access issues
- **Cross-platform**: Built with cross-platform compatibility in mind
- **Extensible**: Easy to add support for additional file types by modifying the minification logic

