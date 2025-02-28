# SrcBundler

## Overview
SrcBundler is a command-line tool written in Rust that recursively scans the `src/` directory of a web development project, extracts JavaScript, TypeScript, JSX, TSX, and CSS files, minifies them, and outputs the concatenated result in a structured format.

## Features
- Supports JavaScript (`.js`), TypeScript (`.ts`), JSX (`.jsx`), TSX (`.tsx`), and CSS (`.css`).
- Option to retain comments using `--keep-comments`.
- Displays file processing statistics (total files, character count).
- Option to copy the final minified content to the clipboard.
- Option to save the output to a file using `--save <filename>.txt`.

## Installation
Ensure you have Rust installed on your system. Then, clone this repository and build the project:

```sh
cargo build --release
```

## Usage
Navigate to the project directory and run:

```sh
./srcbundler
```

### Available Flags
- `--keep-comments`: Retains comments in the minified output.
- `--save <filename>.txt`: Saves the output to a specified file.

### Example Commands
Minify and display output:
```sh
./srcbundler
```

Minify while retaining comments:
```sh
./srcbundler --keep-comments
```

Save minified output to `output.txt`:
```sh
./srcbundler --save output.txt
```

Copy the output to the clipboard:
- When prompted, press `C` to copy the full minified content.

## Notes
- **Currently supports only web development files** (JS, TS, JSX, TSX, CSS).
- Future versions may include support for additional file types.

