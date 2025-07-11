use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
    pub expanded: bool,
}

impl FileNode {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let is_dir = path.is_dir();

        Self {
            path,
            name,
            is_dir,
            children: Vec::new(),
            expanded: false,
        }
    }

    pub fn load_children(&mut self) -> Result<()> {
        if !self.is_dir || !self.children.is_empty() {
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.path)?;
        let mut children = Vec::new();

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // Skip hidden files and common ignore patterns
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with('.')
                    || name_str == "target"
                    || name_str == "node_modules"
                    || name_str == ".git"
                {
                    continue;
                }
            }

            children.push(FileNode::new(path));
        }

        // Sort: directories first, then files, both alphabetically
        children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        self.children = children;
        Ok(())
    }

    pub fn get_display_lines(&self, depth: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let indent = "  ".repeat(depth);
        let prefix = if self.is_dir {
            if self.expanded {
                "📂 "
            } else {
                "📁 "
            }
        } else {
            "📄 "
        };

        lines.push(format!("{}{}{}", indent, prefix, self.name));

        if self.expanded {
            for child in &self.children {
                lines.extend(child.get_display_lines(depth + 1));
            }
        }

        lines
    }
}

pub struct FileExplorer {
    pub root: FileNode,
    pub current_path: PathBuf,
    pub selected_index: usize,
}

impl FileExplorer {
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        let root = FileNode::new(current_dir.clone());

        Ok(Self {
            root,
            current_path: current_dir,
            selected_index: 0,
        })
    }

    pub fn get_display_lines(&self) -> Vec<String> {
        self.root.get_display_lines(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_filenode_new_for_current_dir() {
        let cwd = env::current_dir().unwrap();
        let node = FileNode::new(cwd.clone());
        assert_eq!(node.name, cwd.file_name().unwrap().to_string_lossy());
        assert!(node.is_dir);
    }

    #[test]
    fn test_filenode_new_for_file() {
        let file = "test_file_explorer.txt";
        fs::write(file, "test").unwrap();
        let node = FileNode::new(file.into());
        assert_eq!(node.name, "test_file_explorer.txt");
        assert!(!node.is_dir);
        fs::remove_file(file).unwrap();
    }
}
