# 🚀 ZeroEnv

[![License: MIT](https://shields.io)](https://opensource.org)
[![Rust](https://shields.io)](https://rust-lang.org)
[![PRs Welcome](https://shields.io)](http://makeapullrequest.com)
   [![Crates.io](https://img.shields.io/crates/v/zeroenv.svg)](https://crates.io/crates/zeroenv)
   [![Downloads](https://img.shields.io/crates/d/zeroenv.svg)](https://crates.io/crates/zeroenv)
   
**ZeroEnv** is a zero-configuration, lightning-fast development environment manager written in Rust. 

Inspired by the structural simplicity of Git, `zeroenv` eliminates the friction of local environment setups. Instead of writing heavy `Dockerfiles` or learning complex `Nix` expressions, `zeroenv` automatically scans your repository, detects required toolchains, and locks the versions into a lightweight configuration.

---

## ✨ Features

- **Zero Configuration:** No hand-crafted configs required. It infers everything from your code.
- **Ultra Lightweight:** Compiled into a single, dependency-free binary with zero daemon overhead.
- **Language Agnostic:** Built to support multiple ecosystems out of the box.
- **Deterministic:** Locks local environment states via a simple `.zeroenv` state file.

---

## 🛠️ How It Works

`zeroenv` acts as an automated detective for your project dependencies:

1. **`zeroenv init`** – Scans the current directory for project markers (`package.json`, `go.mod`, etc.), extracts engine/runtime constraints, and generates a unified `.zeroenv` manifest.
2. **`zeroenv status`** – Reads the local manifest and outputs the exact runtime state required for the current workspace.

---

## 📦 Installation

Currently, `zeroenv` is in active MVP stage and can be built from source.

### Prerequisites
Make sure you have the Rust toolchain installed (`cargo`, `rustc`).

### Building from source
```bash
# Clone the repository
git clone https://github.com/nexus-forg/zeroenv.git

# Navigate to the directory
cd zeroenv

# Build the optimized production binary
cargo build --release
```

The compiled binary will be available at `./target/release/zeroenv`. You can move it to your local `PATH` (e.g., `/usr/local/bin/`) for global access.

---

## 🏃‍♂️ Usage

Go to any of your software projects (Node.js, Go, Python, or Rust) and run:

```bash
# Initialize and scan environment
zeroenv init

# View currently locked project environment
zeroenv status
```
### Diagnose and fix environment issues

```bash
$ zeroenv doctor
🩺 zeroenv doctor: диагностика окружения...

❌ Node.js: NOT INSTALLED (требуется >=18.0.0)
💡 FIX для Node.js:
   🐧 Linux:   sudo apt update && sudo apt install nodejs npm
   🍎 macOS:   brew install node
   🪟 Windows: winget install OpenJS.NodeJS.LTS

💡 После установки зависимостей запустите 'zeroenv check', чтобы убедиться, что всё работает.
---

## 🗺️ Roadmap & Ecosystem Status

We are building `zeroenv` incrementally. Here is the current support status:

### Ecosystem Detectors
- [x] **Node.js** (via `package.json` engines parsing)
- [x] **Go** (via `go.mod` version line extraction)
- [x] **Python** (via basic `requirements.txt` parsing)
- [x] **Rust** (via `Cargo.toml` project discovery)
- [ ] **PHP** (via `composer.json`) *(Planned)*
- [ ] **Ruby** (via `Gemfile`) *(Planned)*

### Core Architecture
- [x] Local configuration serialization (`.zeroenv`)
- [ ] Automatic download of missing language binaries into `~/.zeroenv/bin/` *(Next major step)*
- [ ] Context-aware shell mutation (Auto-activating environments on `cd`)
- [ ] Lightweight sandboxing / OS-level process isolation

---

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
