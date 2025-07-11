use crate::{editor::Editor, file_explorer::FileExplorer};
use anyhow::Result;
use crossterm::event::KeyEvent;
use ropey::Rope;
use std::path::PathBuf;

pub struct App {
    pub tabs: Vec<Tab>,
    pub current_tab: usize,
    pub file_explorer: FileExplorer,
    pub show_file_explorer: bool,
    pub show_help: bool,
    pub status_message: Option<String>,
    pub status_timer: u64,
}

pub struct Tab {
    pub path: Option<PathBuf>,
    pub content: Rope,
    pub editor: Editor,
    pub modified: bool,
    pub name: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = Self {
            tabs: Vec::new(),
            current_tab: 0,
            file_explorer: FileExplorer::new()?,
            show_file_explorer: true,
            show_help: false,
            status_message: None,
            status_timer: 0,
        };

        // Create initial empty tab
        app.new_file();
        Ok(app)
    }

    pub fn new_file(&mut self) {
        let tab = Tab {
            path: None,
            content: Rope::from(""),
            editor: Editor::new(),
            modified: false,
            name: "Untitled".to_string(),
        };
        self.tabs.push(tab);
        self.current_tab = self.tabs.len() - 1;
        self.set_status_message("New file created".to_string());
    }

    pub fn open_file_dialog(&mut self) {
        // For now, we'll just create a new file
        // In a full implementation, you'd show a file picker
        self.new_file();
    }

    pub fn save_current_file(&mut self) -> Result<()> {
        if let Some(tab) = self.tabs.get_mut(self.current_tab) {
            if let Some(path) = &tab.path {
                let content = tab.content.to_string();
                std::fs::write(path, content)?;
                tab.modified = false;
                let message = format!("Saved {}", path.display());
                self.set_status_message(message);
            } else {
                // TODO: Implement save as dialog
                self.set_status_message("Save as not implemented yet".to_string());
            }
        }
        Ok(())
    }

    pub fn toggle_panel(&mut self) {
        self.show_file_explorer = !self.show_file_explorer;
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        if let Some(tab) = self.tabs.get_mut(self.current_tab) {
            tab.editor.handle_input(key, &mut tab.content);
            tab.modified = true;
        }
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_timer = 0;
    }

    pub fn get_current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.current_tab)
    }

    pub fn open_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<()> {
        let path = path.into();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        let tab = Tab {
            path: Some(path.clone()),
            content: Rope::from(content),
            editor: Editor::new(),
            modified: false,
            name,
        };
        self.tabs.push(tab);
        self.current_tab = self.tabs.len() - 1;
        self.set_status_message(format!("Opened file: {}", path.display()));
        Ok(())
    }

    pub fn set_directory<P: Into<PathBuf>>(&mut self, dir: P) -> Result<()> {
        let dir = dir.into();
        self.file_explorer.current_path = dir.clone();
        self.file_explorer.root = crate::file_explorer::FileNode::new(dir);
        self.file_explorer.root.load_children()?;
        self.file_explorer.root.expanded = true;
        Ok(())
    }
}
