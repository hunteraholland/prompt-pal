mod cli;
mod fileinfo;
mod tokenizer;
mod walkdir;

use clap::Parser;
use cli::Cli;
use fileinfo::FileInfo;
use std::error::Error;
use tokenizer::count_tokens;
use walkdir::scan_directory;

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
        let file_info = FileInfo::new(file, 0)?;

        if cli.tokens {
            let (token_count, _) =
                count_tokens(&file_info.content_preview.clone().unwrap_or_default(), None);
            println!("File: {}, Token count: {}", file.display(), token_count);
        }

        results.push(file_info);
    }

    // Generate XML if requested
    if cli.xml {
        let xml = generate_xml(&results);
        if let Some(output_path) = cli.output {
            std::fs::write(&output_path, xml)?;
            println!("XML output written to: {}", output_path.display());
        } else {
            println!("XML output:\n{}", xml);
        }
    }

    Ok(())
}

fn generate_xml(files: &[FileInfo]) -> String {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<files>\n");

    for file in files {
        xml.push_str(&format!("  <file>\n"));
        xml.push_str(&format!("    <path>{}</path>\n", file.path.display()));
        xml.push_str(&format!("    <size>{}</size>\n", file.size));
        xml.push_str(&format!("  </file>\n"));
    }

    xml.push_str("</files>");
    xml
}
