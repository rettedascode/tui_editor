# TUI Code Editor

A terminal-based code editor written in Rust with file and folder support.

## Features

- **Multi-tab editing**: Open multiple files in tabs
- **File explorer**: Browse and navigate through files and folders
- **Line numbers**: Shows line numbers for easy navigation
- **Status bar**: Displays cursor position, line count, and file information
- **Keyboard shortcuts**: Full keyboard navigation and editing
- **Help system**: Built-in help screen (F1)

## Installation

### Windows 10/11

1. Open PowerShell
2. Run:
   ```powershell
   .\install.ps1
   ```
   This will install Rust (if missing), build the project, and copy the binary to your cargo bin directory.

### Linux

1. Open a terminal
2. Run:
   ```bash
   bash ./install.sh
   ```
   This will install Rust (if missing), build the project, and copy the binary to your cargo bin directory.

**Note:**
- On Windows, run each command on its own line in PowerShell. Do not use `&&` for chaining commands.
- On Linux/macOS, you can use `&&` to chain commands.

## Usage

### CLI Options
You can launch the editor with:
- `tui_editor -f <file>` to open a specific file
- `tui_editor -d <directory>` to open a specific directory in the file explorer
- You can combine them: `tui_editor -d code/ -f code/main.rs`

### Navigation
- **Arrow Keys**: Move cursor
- **Home/End**: Move to beginning/end of line
- **Page Up/Down**: Page navigation
- **Tab**: Toggle file explorer panel

### File Operations
- **Ctrl+N**: Create new file
- **Ctrl+O**: Open file (placeholder - creates new file)
- **Ctrl+S**: Save current file
- **Q**: Quit the editor

### Editor Features
- **F1**: Toggle help screen
- **Backspace/Delete**: Delete characters
- **Enter**: Insert new line
- **All printable characters**: Insert text

### File Explorer
- The file explorer shows the current directory structure
- Files and folders are displayed with icons
- Hidden files and common ignore patterns (target, node_modules, .git) are filtered out
- Directories are shown first, then files, both sorted alphabetically

## Project Structure

```
src/
├── main.rs          # Main application entry point
├── app.rs           # Application state management
├── editor.rs        # Text editor functionality
├── file_explorer.rs # File system browser
└── ui.rs           # User interface rendering
```

## Dependencies

- **ratatui**: Terminal UI framework
- **crossterm**: Cross-platform terminal manipulation
- **ropey**: Efficient text rope data structure
- **walkdir**: Directory traversal
- **anyhow**: Error handling
- **syntect**: Syntax highlighting (optional)
- **clap**: Command-line argument parsing

## Key Features Explained

### Text Editing
The editor uses the `ropey` crate for efficient text manipulation. This allows for:
- Fast insertion and deletion at any position
- Efficient line-based operations
- Memory-efficient storage of large files

### File Explorer
The file explorer provides:
- Tree-like view of the file system
- Expandable/collapsible directories
- File filtering (hidden files, build directories)
- Visual indicators for files vs directories

### Multi-tab Support
- Each tab maintains its own editor state
- Tabs show modification status with asterisk (*)
- Current tab is highlighted with an arrow (▶)

### Status Bar
The status bar displays:
- Current cursor position (line, column)
- Total lines and characters in the file
- Status messages for operations
- File information

## Future Enhancements

- [ ] Syntax highlighting for different file types
- [ ] Find and replace functionality
- [ ] Copy/paste support
- [ ] Undo/redo functionality
- [ ] File search and filtering
- [ ] Multiple cursors
- [ ] Split views
- [ ] Plugin system
- [ ] Configuration file support
- [ ] Themes and color schemes

## Contributing

Feel free to contribute to this project by:
- Reporting bugs
- Suggesting new features
- Submitting pull requests
- Improving documentation

## License

This project is open source and available under the MIT License.
