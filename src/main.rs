mod cli;
mod fileinfo;
mod tokenizer;
mod walkdir;
mod xml;

use clap::Parser;
use cli::Cli;
use fileinfo::FileInfo;
use std::error::Error;
use tokenizer::count_tokens;
use walkdir::scan_directory;
use xml::XmlGenerator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Enable debug logging if requested
    if cli.debug > 0 {
        println!("Debug mode enabled (level: {})", cli.debug);
    }

    // Scan the directory
    println!("Scanning directory: {}", cli.directory.display());
    let files = scan_directory(&cli.directory)?;

    // Process each file
    let mut results = Vec::new();
    for file in files.iter() {
        let file_info = if cli.xml {
            FileInfo::with_full_content(file)? // Full content for XML output
        } else {
            FileInfo::with_preview(file, 1024)? // Preview for token counting only
        };

        if cli.tokens {
            let (token_count, _) =
                count_tokens(&file_info.content.clone().unwrap_or_default(), None);
            println!("File: {}, Token count: {}", file.display(), token_count);
        }

        results.push(file_info);
    }

    // Generate XML if requested
    if cli.xml {
        let xml = XmlGenerator::generate(&results);
        if let Some(output_path) = cli.output {
            std::fs::write(&output_path, xml)?;
            println!("XML output written to: {}", output_path.display());
        } else {
            println!("XML output:\n{}", xml);
        }
    }

    Ok(())
}
