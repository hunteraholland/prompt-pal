use crate::fileinfo::FileInfo;

pub struct XmlGenerator;

impl XmlGenerator {
    pub fn generate(files: &[FileInfo]) -> String {
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
} 