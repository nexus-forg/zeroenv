<div align="center">

**Zero-configuration development environment manager**

[![Crates.io](https://img.shields.io/crates/v/zeroenv.svg)](https://crates.io/crates/zeroenv)
[![Downloads](https://img.shields.io/crates/d/zeroenv.svg)](https://crates.io/crates/zeroenv)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

[Quick Start](#quick-start) • [Features](#features) • [Installation](#installation) • [Usage](#usage) 

</div>

---

## 🎯 What is zeroenv?

**zeroenv** eliminates the friction of local environment setups. Instead of writing heavy Dockerfiles or learning complex Nix expressions, zeroenv automatically scans your repository, detects required toolchains, and locks the versions into a lightweight configuration.

Inspired by the structural simplicity of Git, zeroenv is:
- **Zero Configuration**: No hand-crafted configs required. It infers everything from your code.
- **Ultra Lightweight**: Compiled into a single, dependency-free binary with zero daemon overhead.
- **Language Agnostic**: Built to support multiple ecosystems out of the box.
- **Deterministic**: Locks local environment states via a simple `.zeroenv` state file.

---

# 🚀 Quick Start

```bash
# Install zeroenv
cargo install zeroenv

# Scan your project and generate .zeroenv
zeroenv init

# Check if your system meets requirements
zeroenv check

# Get OS-specific installation commands for missing tools
zeroenv doctor
```

That's it. No YAML files, no Docker, no complex setup.

---

## ✨ Features

### 🔍 Automatic Detection
Scans your project directory and detects:
- **Node.js**: Parses `package.json` for engine requirements.
- **Go**: Extracts version from `go.mod`.
- **Python**: Reads dependencies from `requirements.txt` or `pyproject.toml`.
- **Rust**: Detects `Cargo.toml` projects.

### 🩺 Smart Diagnostics
The `doctor` command doesn't just tell you what's missing — it provides the exact, OS-specific package manager commands to install them instantly.
$ zeroenv doctor
🩺 zeroenv doctor: диагностика окружения...

❌ Node.js: NOT INSTALLED (требуется >=18.0.0)
💡 FIX для Node.js:
   🐧 Linux:   sudo apt update && sudo apt install nodejs npm
   🍎 macOS:   brew install node
   🪟 Windows: winget install OpenJS.NodeJS.LTS

❌ Go: NOT INSTALLED (требуется 1.21)
💡 FIX для Go:
   🐧 Linux:   sudo apt install golang-go
   🍎 macOS:   brew install go
   🪟 Windows: winget install GoLang.Go

💡 После установки зависимостей запустите 'zeroenv check', чтобы убедиться, что всё работает.

📦 Lightweight State File
The .zeroenv file is simple, human-readable, and easy to version control:

NODE_VERSION=>=18.0.0
GO_VERSION=1.21
PYTHON_DEPS=Django==4.2.0, requests>=2.28.0
RUST_PROJECT=true

📥 Installation
From crates.io (Recommended)

cargo install zeroenv

From Source:
git clone https://github.com/nexus-forg/zeroenv.git
cd zeroenv
cargo build --release
./target/release/zeroenv
System Requirements

    Rust 1.70 or higher (for building from source)
    No runtime dependencies (compiled binary works out of the box)

📖 Usage
zeroenv init
Scans the current directory and generates a .zeroenv configuration file:

$ zeroenv init
Scanning project directory...
Detected: Node.js project
  Required Node.js: >=18.0.0
Detected: Go project
  Required Go: 1.21
Detected: Python project
  Dependencies: Django==4.2.0, requests>=2.28.0
Detected: Rust project
Configuration successfully saved to .zeroenv

zeroenv check

Validates your system against the project requirements:

$ zeroenv check
🔍 Проверяю окружение...

✅ Node.js: OK (v20.10.0 satisfies >=18.0.0)
✅ Go: OK (1.21.5 satisfies 1.21)
✅ Python: INSTALLED (version 3.11.5)
   📦 Dependencies from .zeroenv: Django==4.2.0, requests>=2.28.0
✅ Rust: INSTALLED (rustc 1.75.0)

💡 Проверка завершена.

zeroenv doctor

Diagnoses issues and provides OS-specific installation commands:

$ zeroenv doctor
🩺 zeroenv doctor: диагностика окружения...

❌ Node.js: NOT INSTALLED (требуется >=18.0.0)
💡 FIX для Node.js:
   🐧 Linux:   sudo apt update && sudo apt install nodejs npm
   🍎 macOS:   brew install node
   🪟 Windows: winget install OpenJS.NodeJS.LTS

💡 После установки зависимостей запустите 'zeroenv check', чтобы убедиться, что всё работает.

zeroenv status

$ zeroenv status
Current project environment (.zeroenv):
NODE_VERSION=>=18.0.0
GO_VERSION=1.21
PYTHON_DEPS=Django==4.2.0, requests>=2.28.0
RUST_PROJECT=true

🏗️ Architecture
zeroenv is built with a clean, modular architecture:

src/
├── main.rs       # CLI interface and command routing
├── config.rs     # .zeroenv file reading/writing
├── scanner.rs    # Project manifest parsing
└── checker.rs    # System validation and diagnostics

This design makes it easy to add support for new languages and ecosystems.

🗺️ Roadmap

    Terminal color output (green for OK, red for errors)
    Git hooks integration (auto-check on git checkout)
    Support for PHP (composer.json)
    Support for Ruby (Gemfile)
    Support for Java (pom.xml, build.gradle)
    Interactive mode for version selection

🤝 Contributing
Contributions are welcome! If you have ideas for new features, bug fixes, or improvements:

    Fork the repository
    Create your feature branch (git checkout -b feature/amazing-feature)
    Commit your changes (git commit -m 'Add amazing feature')
    Push to the branch (git push origin feature/amazing-feature)
    Open a Pull Request

Please open an issue first to discuss major changes.

📄 License
This project is licensed under the MIT License - see the LICENSE
 file for details.
🙏 Acknowledgments

    Built with Rust
    Inspired by the simplicity of Git
    Published on crates.io

<div align="center">

Made with ❤️ by developers, for developers
⭐ Star this repo
 if you find it useful!
</div>

