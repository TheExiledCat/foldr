# foldr — Directory Template Generator CLI Tool

## Project Overview

`foldr` is a Linux CLI tool to create, manage, and instantiate directory templates efficiently. Inspired by dotnet project templates but generalized for *any* folder structure, `foldr` lets you snapshot a folder into a compressed template and recreate it instantly anywhere.

**Key features:**

- Save any directory as a reusable template  
- Instantiate templates quickly with the original folder structure and files  
- Store templates compressed as ZIP archives for space efficiency  
- Support multiple versions per template (with hashes or version tags)  
- Configurable storage location (default `~/.foldr`)  
- Optional TUI interface to browse/search templates and subfolders  
- Future plans for variable substitution and more advanced templating  

---

## Folder Structure

```
foldr/
├── src/
│   ├── cli.rs         # CLI parsing and commands (using clap)
│   ├── tui.rs         # TUI interface (using ratatui)
│   ├── templates.rs   # Template management: save, load, list, purge
│   ├── zip.rs         # ZIP compression and decompression utilities
│   ├── config.rs      # Config file and flag management
│   └── main.rs        # Entrypoint: initialize CLI or TUI modes
├── Cargo.toml
├── README.md
└── LICENSE
```

---

## Features & Roadmap

### v0.1 - MVP

- Basic CLI:  
  - `foldr save <folder> --name <template_name>`  
  - `foldr create <template_name> [destination]`  
  - `foldr list`  
  - `foldr purge <template_name> [--version <version>]`  
- Store templates as zipped archives in `~/.foldr`  
- Config support to override default storage path  
- Handle current directory as default folder to save  
- Simple error handling and logging  

### v0.2 - Improvements

- Add versioning support:  
  - Assign hash or version tag to each saved template snapshot  
  - Allow purging old versions by version or retention policy  
- Support optional compiler flag `--cli-only` to exclude TUI in builds  
- Improve CLI UX and output formatting  

### v0.3 - TUI Mode

- Interactive terminal UI to browse templates  
- Search and filter templates by name or tags  
- Select subdirectories within templates for selective creation  
- Use `ratatui` for widget-based UI design  
- Add keybindings, help, and smooth navigation  

### v0.4+ - Future Extensions

- Variable substitution within templates (e.g., project name, author)  
- Support for multiple template sources (local, git repositories)  
- Template sharing and publishing commands  
- Extend configuration options (profiles, aliases)  
- Cross-platform support beyond Linux  

---

## Development Guide & ToDos

### Setup

- Use Rust (stable toolchain)  
- Dependencies:  
  - `clap` for CLI parsing  
  - `ratatui` for TUI interface  
  - `zip` crate for archive compression  
  - `serde` + `toml` or `yaml` for config files  

### Main CLI commands to implement

- `save` - save folder snapshot as a new template  
- `create` - instantiate a template copy  
- `list` - show available templates and versions  
- `purge` - delete old or unwanted templates  

### TUI features to build

- Main menu: list templates with search box  
- Template detail view: browse folders and files inside archive  
- Create new instance with directory selection  
- Keyboard navigation and help panel  

### Architecture Notes

- Maintain a global config file at `~/.foldr/config.toml` for defaults  
- Templates stored in `~/.foldr/templates/<template_name>/<version>.zip`  
- Use zip file internal directory structure for template data  
- Consider streaming unzip to reduce memory usage  

---

## Example Usage

```bash
# Save current dir as template "cproject"
foldr save . --name cproject

# List saved templates
foldr list

# Create new folder from "cproject" template in ./myapp
foldr create cproject ./myapp

# Launch interactive TUI
foldr tui
```

---

## License

MIT License

---

*Happy coding! 🚀*
