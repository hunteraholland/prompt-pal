# PromptPal 🤖

PromptPal is a command-line tool that helps developers create well-structured prompts for Large Language Models (LLMs) by intelligently incorporating repository context. It streamlines the process of getting high-quality AI assistance with code-related tasks.

## Features

- 📁 Automatically analyzes repository structure
- 🎯 Generates structured prompts with relevant code context
- 💡 Optimizes prompts for better LLM responses
- ⚡ Simple command-line interface
- 🔍 Token counting for LLM context limits
- 📊 XML output with language-aware code blocks

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Commands

1. Scan a directory:
```bash
promptpal --directory ./prompts
```

2. Count tokens in files:
```bash
promptpal --directory ./prompts --tokens
```

3. Generate XML output with instructions:
```bash
promptpal --directory ./prompts --xml --instructions "These files implement the authentication system"
```

4. Save XML output to file:
```bash
promptpal --directory ./prompts --xml --output results.xml
```

### Options

- `-d, --directory <DIR>`: Directory to scan for prompt files
- `-n, --instructions <INSTRUCTIONS>`: Add instructions to the XML output
- `-t, --tokens`: Count tokens in each file
- `-x, --xml`: Generate XML output
- `-o, --output <FILE>`: Save output to file (optional)
- `-v, --debug`: Enable debug logging (can be repeated for more verbosity)

### XML Output Format

The XML output includes:
- Instructions (if provided)
- File paths
- File sizes
- File contents (complete or preview)
- Language-specific code blocks

Example output:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<files>
  <instructions>
    <![CDATA[
    These files contain the core business logic for user authentication.
    ]]>
  </instructions>
  <file>
    <path>example.rs</path>
    <size>100</size>
    <content complete="true" type="rs">
      ```rs
      fn main() {
          println!("Hello, World!");
      }
      ```
    </content>
  </file>
</files>
```

## Why PromptPal?

When working with AI coding assistants, the quality of responses heavily depends on providing proper context. PromptPal simplifies this process by:

- Automatically identifying relevant files and code snippets
- Structuring prompts in a format that LLMs can understand well
- Saving developers time on prompt engineering
- Ensuring consistent prompt quality across team members
- Providing token counts to stay within LLM context limits
- Generating XML output with language-aware code blocks

## Development

Run tests:
```bash
cargo test
```

Build in release mode:
```bash
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust 🦀
- Inspired by the need for better AI-assisted development workflows