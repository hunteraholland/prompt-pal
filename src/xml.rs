use crate::fileinfo::FileInfo;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
struct FileNode {
    name: String,
    file_info: Option<FileInfo>,
    children: HashMap<String, FileNode>,
}

pub struct XmlGenerator;

impl XmlGenerator {
    pub fn generate(files: &[FileInfo], instructions: &str) -> String {
        let root = Self::build_tree(files);
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<files>\n");

        // Add user instructions
        xml.push_str("  <instructions>\n    <![CDATA[\n");
        xml.push_str(instructions);
        xml.push_str("\n    ]]>\n  </instructions>\n\n");

        // Generate and add the tree visualization
        let tree_view = Self::generate_tree_view(&root);
        xml.push_str("  <tree_view>\n");
        for line in tree_view.lines() {
            xml.push_str(&format!("    {}\n", Self::escape_xml(line)));
        }
        xml.push_str("  </tree_view>\n\n");

        Self::generate_xml_from_tree(&root, &mut xml, 1);
        xml.push_str("</files>");
        xml
    }

    fn generate_tree_view(node: &FileNode) -> String {
        let mut result = String::new();
        Self::generate_tree_view_recursive(node, "", "", &mut result);
        result
    }

    fn generate_tree_view_recursive(
        node: &FileNode,
        prefix: &str,
        name: &str,
        result: &mut String,
    ) {
        if !name.is_empty() {
            result.push_str(&format!("{}{}\n", prefix, name));
        }

        let mut sorted_children: Vec<_> = node.children.values().collect();
        sorted_children.sort_by(|a, b| a.name.cmp(&b.name));

        for (i, child) in sorted_children.iter().enumerate() {
            let is_last = i == sorted_children.len() - 1;
            let (next_prefix, pointer) = if is_last {
                (format!("{}    ", prefix), "└── ")
            } else {
                (format!("{}│   ", prefix), "├── ")
            };

            let display_name = if child.file_info.is_some() {
                &child.name
            } else {
                &format!("{}/", child.name)
            };

            Self::generate_tree_view_recursive(
                child,
                &next_prefix,
                &format!("{}{}", pointer, display_name),
                result,
            );
        }
    }

    fn build_tree(files: &[FileInfo]) -> FileNode {
        let mut root = FileNode::default();

        for file in files {
            let path_components: Vec<_> = file
                .path
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            let mut current = &mut root;

            // Create the path hierarchy
            for (i, component) in path_components.iter().enumerate() {
                let is_last = i == path_components.len() - 1;
                current = current
                    .children
                    .entry(component.clone())
                    .or_insert_with(|| FileNode {
                        name: component.clone(),
                        file_info: if is_last { Some(file.clone()) } else { None },
                        children: HashMap::new(),
                    });
            }
        }

        root
    }

    fn generate_xml_from_tree(node: &FileNode, xml: &mut String, depth: usize) {
        let indent = "  ".repeat(depth);

        if let Some(file_info) = &node.file_info {
            xml.push_str(&format!("{}<file>\n", indent));
            xml.push_str(&format!(
                "{}  <path>{}</path>\n",
                indent,
                file_info.path.display()
            ));
            xml.push_str(&format!("{}  <size>{}</size>\n", indent, file_info.size));

            if let Some(content) = &file_info.content {
                let file_type = Self::detect_file_type(&file_info.path);
                xml.push_str(&format!(
                    "{}  <content complete=\"{}\" type=\"{}\">\n{}    <![CDATA[\n",
                    indent, file_info.is_content_complete, file_type, indent
                ));

                // Add content without indentation
                xml.push_str(content);
                xml.push_str(&format!("\n{}    ]]>\n{}  </content>\n", indent, indent));
            }
            xml.push_str(&format!("{}</file>\n", indent));
        }

        // Sort children for consistent output
        let mut sorted_children: Vec<_> = node.children.values().collect();
        sorted_children.sort_by(|a, b| a.name.cmp(&b.name));

        for child in sorted_children {
            Self::generate_xml_from_tree(child, xml, depth);
        }
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
