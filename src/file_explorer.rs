use anyhow::Result;
use std::path::{Path, PathBuf};

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
                if name_str.starts_with('.') || 
                   name_str == "target" || 
                   name_str == "node_modules" ||
                   name_str == ".git" {
                    continue;
                }
            }

            children.push(FileNode::new(path));
        }

        // Sort: directories first, then files, both alphabetically
        children.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        self.children = children;
        Ok(())
    }

    pub fn toggle_expanded(&mut self) -> Result<()> {
        if self.is_dir {
            if self.expanded {
                self.expanded = false;
            } else {
                self.load_children()?;
                self.expanded = true;
            }
        }
        Ok(())
    }

    pub fn get_display_lines(&self, depth: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let indent = "  ".repeat(depth);
        let prefix = if self.is_dir {
            if self.expanded { "📂 " } else { "📁 " }
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

    pub fn refresh(&mut self) -> Result<()> {
        self.root = FileNode::new(self.current_path.clone());
        self.root.load_children()?;
        self.root.expanded = true;
        Ok(())
    }

    pub fn get_all_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        self.collect_files(&self.root, &mut files);
        files
    }

    fn collect_files(&self, node: &FileNode, files: &mut Vec<PathBuf>) {
        if !node.is_dir {
            files.push(node.path.clone());
        } else if node.expanded {
            for child in &node.children {
                self.collect_files(child, files);
            }
        }
    }

    pub fn get_display_lines(&self) -> Vec<String> {
        self.root.get_display_lines(0)
    }

    pub fn select_file(&mut self, index: usize) -> Option<PathBuf> {
        let files = self.get_all_files();
        if index < files.len() {
            self.selected_index = index;
            Some(files[index].clone())
        } else {
            None
        }
    }

    pub fn get_selected_file(&self) -> Option<PathBuf> {
        let files = self.get_all_files();
        files.get(self.selected_index).cloned()
    }

    pub fn move_selection(&mut self, direction: i32) {
        let files = self.get_all_files();
        if files.is_empty() {
            return;
        }

        let new_index = if direction > 0 {
            (self.selected_index + 1) % files.len()
        } else {
            if self.selected_index == 0 {
                files.len() - 1
            } else {
                self.selected_index - 1
            }
        };

        self.selected_index = new_index;
    }

    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        if path.is_file() {
            // File will be opened by the main app
            Ok(())
        } else if path.is_dir() {
            self.current_path = path.to_path_buf();
            self.refresh()?;
            self.selected_index = 0;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Path does not exist"))
        }
    }

    pub fn get_file_info(&self, path: &Path) -> Result<FileInfo> {
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();
        let modified = metadata.modified()?;
        
        Ok(FileInfo {
            size,
            modified,
            is_readonly: metadata.permissions().readonly(),
        })
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub is_readonly: bool,
} 