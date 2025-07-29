# foldr

`foldr` is a blazing-fast, flexible command-line tool written in Rust for managing reusable **directory / project templates**.  
Easily snapshot a folder structure and reuse it anytime — great for setting up projects, configurations, or boilerplate.

---

## 🚀 Features

- 📦 Save any folder as a reusable template  
- 🧱 Recreate folder structure and files with one command  
- 🗃️ Templates are compressed as ZIPs for efficient storage  
- 🧭 Supports versioning per template  
- 🔧 Configurable storage location (default: `~/.foldr`)  
- 🖥️ Optional terminal UI using `ratatui`  
- 🛠️ Future support for variable substitution and remote templates  

---

## 📦 Installation



```bash
# Using Cargo
cargo install foldr
```






Or build from source using cargo toolchain:

```bash
git clone https://github.com/yourname/foldr.git
cd foldr
cargo build --release 
# or cargo build --release --features tui if you want tui support
```

You can also grab a prebuilt binary for your platform in the releases page

---

## 🛠️ Usage

```bash
# Save current directory as a template named 'cproject'

foldr save . cproject
# or group templates (this will make a directory called c and place the template in it)
foldr save . c/helloworld

# List all saved templates

foldr list

# Create a new project from template 'cproject'

foldr new cproject ./new_project

# Launch the terminal UI for browing and editing templates (only available when using the tui feature flag or from a prebuilt binary)

foldr tui

# update a template to the current directory and increase its version counter
foldr update cproject . 

# delete old versions of all or a single template
foldr purge [Template]

# show the contents of a template 
foldr show cproject

# http support for fetching remote templates and easy sharing of templates
foldr new https://example.com/template.foldr ./remote_project
# or to create a permanent template from a remote
foldr fetch https://example.com/template.foldr template_name
```



---

## 📁 Template Storage

- Templates are stored in: 
  - `~/.foldr/templates/` on linux and mac 
  - `%USERPROFILE%\foldr\templates` on windows
- Each template is saved as a `.foldr` file with versioning support (its really just a zip file containing the folder and also some metadata)

---

## ⚙️  Configuration

- configurations are stored in `~/.foldr/config.json` or the file stored in the `--config` flag
- the config file can be created using the `foldr config` command which will create the configuration using an interactive cli
- for config options see [Configuration](Configuration.md)
- Note: By default foldr uses a small sqlite database cache to speed up searching and listing for templates. This can be disabled with the `use_cache` key in the config. When disabled searching and listing will manually walk the templates directory and list the templates that way, which is much slower if you have alot of templates.

---

## 🔮 Roadmap

- [ ] CLI with save, new, update, list, purge  
- [ ] TUI to browse/search templates and preview / edit them
- [ ] Template variable substitution (creating files/directories with variable names)
- [ ] Support for templates from remote sources  
- [ ] Publishing templates for sharing  

---

## 🤝 Contributing

Pull requests, issues, and feedback welcome!  
Please check out the `CONTRIBUTING.md` once available.

---

## 📄 License

Licensed under the MIT License. See `LICENSE` for details.

---

Made with 💙 in Rust.
