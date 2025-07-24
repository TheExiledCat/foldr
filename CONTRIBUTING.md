# Contributing to foldr

Thank you for your interest in contributing to **foldr**! Your ideas, improvements, and bug reports are very welcome. This guide will help you get started.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Feature Suggestions](#feature-suggestions)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting a Pull Request](#submitting-a-pull-request)

---

## 📜 Code of Conduct

Please be respectful and kind in all interactions. This project follows the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct.

---

## 🚀 Getting Started

To get the project running locally:

```bash
git clone https://github.com/yourname/foldr.git
cd foldr
cargo build
```

To run it:

```bash
cargo run -- <your_args_here>
```

If you're working on the TUI:

```bash
cargo run -- tui
```

---

## 🤝 How to Contribute

You can contribute in many ways:

- Report bugs
- Request features
- Improve the documentation
- Optimize performance or reduce dependencies
- Submit pull requests for fixes or features (please start a discussion / issue before working on a PR so we can even figure out if the fix is necessary)

---

## 💡 Feature Suggestions

If you have an idea, open a GitHub Issue with the `enhancement` label. Please include:

- A clear title
- Motivation or use case
- Any potential downsides or alternatives

---

## 🧼 Code Style

We follow standard Rust practices:

- Use `rustfmt` to format code:

```bash
cargo fmt
```

- Use `clippy` to catch warnings:

```bash
cargo clippy
```

- Avoid unnecessary dependencies
- Prefer clear and expressive code over clever hacks

---

## ✅ Testing

Run all tests using:

```bash
cargo test
```

Please include tests for new functionality when possible.

---

## 🔀 Submitting a Pull Request

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes
4. Push to your fork
5. Open a pull request

Be descriptive in your PR title and description — include screenshots or example output if applicable.

---

## 🙏 Thanks

Thanks again for considering a contribution to foldr!  
Every little bit helps make this tool better for everyone.

---
