use crate::fileinfo::FileInfo;
use std::path::Path;

pub struct XmlGenerator;

impl XmlGenerator {
    pub fn generate(files: &[FileInfo]) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<files>\n");

        for file in files {
            xml.push_str("  <file>\n");
            xml.push_str(&format!("    <path>{}</path>\n", file.path.display()));
            xml.push_str(&format!("    <size>{}</size>\n", file.size));
            if let Some(content) = &file.content {
                let file_type = Self::detect_file_type(&file.path);
                xml.push_str(&format!(
                    "    <content complete=\"{}\" type=\"{}\">\n      ```{}\n      {}\n      ```\n    </content>\n",
                    file.is_content_complete,
                    file_type,
                    file_type,
                    Self::escape_xml(content)
                ));
            }
            xml.push_str("  </file>\n");
        }

        xml.push_str("</files>");
        xml
    }

    fn detect_file_type(path: &Path) -> String {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .unwrap_or_else(|| "text".to_string())
    }

    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}
