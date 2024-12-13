use crate::fileinfo::FileInfo;

pub struct XmlGenerator;

impl XmlGenerator {
    pub fn generate(files: &[FileInfo]) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<files>\n");

        for file in files {
            xml.push_str(&format!("  <file>\n"));
            xml.push_str(&format!("    <path>{}</path>\n", file.path.display()));
            xml.push_str(&format!("    <size>{}</size>\n", file.size));
            if let Some(content) = &file.content {
                xml.push_str(&format!(
                    "    <content complete=\"{}\">{}</content>\n",
                    file.is_content_complete,
                    Self::escape_xml(content)
                ));
            }
            xml.push_str(&format!("  </file>\n"));
        }

        xml.push_str("</files>");
        xml
    }

    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}
