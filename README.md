# Rust Markdown Compiler

A lightweight and efficient Markdown to HTML compiler written in Rust. This compiler transforms Markdown syntax into clean, semantic HTML.

## Features

### Headers
Supports three levels of headers using the `#` syntax:

- `#` for the highest level (h1)
- `##` for the second level (h2)
- `###` for the third level (h3)

### Bold and Italic Text

Supports bold and italic text using the `**` and `*` syntax:

- `**text**` for bold text
- `*text*` for italic text

### Paragraphs

Automatically wraps text in `<p>` tags when no other tags are present.

### Additional Text Formatting

#### Subscript
Use single tildes: `H~2~O` becomes H<sub>2</sub>O

#### Superscript
Use carets: `2^nd^` becomes 2<sup>nd</sup>

#### Small Text
Use double percent signs: `%%smaller text%%`

#### Marked/Highlighted Text
Use double equals signs: `==highlighted text==`

#### Abbreviations
Use [@text@title] syntax: `[@WHO@World Health Organization]`

## Usage

```bash
cargo run <input_file.md>
```

The compiler will generate an `output.html` file in the same directory as the input file.

## Example

See the `test.md` file for an example of the compiler in action.

## Installation

1. Clone the repository

```bash
git clone https://github.com/awwyushh/md_compiler
```

2. Build your project

```bash
cd md_compiler
cargo build --release
```

3. Run your project
```bash
cargo run test.md
```

## Project Structure

- `src/parse_markdown.rs`: Parses the markdown file and outputs the HTML file.
- `src/main.rs`: The main entry point for the program.
- `test.md`: An example markdown file to test the compiler.
- `output.html`: The output HTML file generated by the compiler.

## Future Features

- [ ] Add support for ordered lists
- [ ] Add support for unordered lists
- [x] Add support for links
- [ ] Add support for images
- [ ] Add support for tables
- [ ] Add support for code blocks
- [x] Add support for blockquotes
- [ ] Add support for horizontal rules
- [ ] Add support for line breaks
- [x] Add support for strikethrough text
- [x] Add support for subscript and superscript text
- [ ] Add support for small text
- [ ] Add support for mark text
- [ ] Add support for abbreviations
- [ ] Add support for subscript and superscript text
- [ ] Add support for small text
- [ ] Add support for mark text
- [ ] Add support for ruby annotations

## License

This project is open-sourced under the MIT License - see the LICENSE file for details.
