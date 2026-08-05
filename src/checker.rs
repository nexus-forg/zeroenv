use std::process::Command;
use std::path::Path;
use std::fs;
use crate::config::parse_zeroenv;

fn get_system_version(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command).args(args).output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn compare_versions(required: &str, actual: &str) -> bool {
    let required_clean = required.trim_start_matches(">=").trim_start_matches('v');
    let actual_clean = actual.trim_start_matches('v');
    
    let required_major = required_clean.split('.').next();
    let actual_major = actual_clean.split('.').next();
    
    if let (Some(req), Some(act)) = (required_major, actual_major) {
        if let (Ok(req_num), Ok(act_num)) = (req.parse::<u32>(), act.parse::<u32>()) {
            return act_num >= req_num;
        }
    }
    false
}

pub fn run_check() {
    let env = parse_zeroenv();
    
    if env.is_empty() {
        println!("❌ Файл .zeroenv не найден или пуст. Запустите сначала 'zeroenv init'");
        return;
    }
    
    println!("🔍 Проверяю окружение...\n");
    
    // Rust
    if env.contains_key("RUST_PROJECT") {
        if let Some(actual) = get_system_version("rustc", &["--version"]) {
            let version = actual.split_whitespace().nth(1).unwrap_or(&actual);
            println!("✅ Rust: INSTALLED (rustc {})", version);
        } else {
            println!("❌ Rust: NOT INSTALLED (project requires Rust toolchain)");
        }
    }

    // Python
    if env.contains_key("PYTHON_DEPS") {
        let python_cmd = if get_system_version("python3", &["--version"]).is_some() {
            "python3"
        } else {
            "python"
        };
    
        if let Some(actual) = get_system_version(python_cmd, &["--version"]) {
            let version = actual.split_whitespace().nth(1).unwrap_or(&actual);
            println!("✅ Python: INSTALLED (version {})", version);
            println!("   📦 Dependencies from .zeroenv: {}", env.get("PYTHON_DEPS").unwrap());
        } else {
            println!("❌ Python: NOT INSTALLED (project has dependencies)");
        }
    }

    // Node.js
    if let Some(required) = env.get("NODE_VERSION") {
        if let Some(actual) = get_system_version("node", &["-v"]) {
            if compare_versions(required, &actual) {
                println!("✅ Node.js: OK ({} satisfies {})", actual, required);
            } else {
                println!("⚠️  Node.js: MISMATCH (project requires {}, but you have {})", required, actual);
            }
        } else {
            println!("❌ Node.js: NOT INSTALLED (project requires {})", required);
        }
    }
    
    // Go
    if let Some(required) = env.get("GO_VERSION") {
        if let Some(actual) = get_system_version("go", &["version"]) {
            let raw_version = actual.split_whitespace().nth(2).unwrap_or(&actual);
            let version = raw_version
                .trim_start_matches("go")
                .split('-')
                .next()
                .unwrap_or(raw_version);
            if compare_versions(required, version) {
                println!("✅ Go: OK ({} satisfies {})", version, required);
            } else {
                println!("⚠️  Go: MISMATCH (project requires {}, but you have {})", required, version);
            }
        } else {
            println!("❌ Go: NOT INSTALLED (project requires {})", required);
        }
    }
    
    println!("\n💡 Проверка завершена.");
}

pub fn run_status() {
    if Path::new(".zeroenv").exists() {
        println!("Current project environment (.zeroenv):");
        if let Ok(content) = fs::read_to_string(".zeroenv") {
            print!("{}", content);
        }
    } else {
        eprintln!("Error: .zeroenv not found. Run 'zeroenv init' first.");
    }
}

pub fn run_doctor() {
    let env = parse_zeroenv();
    if env.is_empty() {
        println!("❌ Файл .zeroenv не найден. Запустите сначала 'zeroenv init'");
        return;
    }
    
    println!("🩺 zeroenv doctor: диагностика окружения...\n");
    let os = std::env::consts::OS;
    let mut issues_found = false;

    let print_fix = |tool: &str, os: &str, linux_cmd: &str, mac_cmd: &str, win_cmd: &str| {
        println!("💡 FIX для {}:", tool);
        match os {
            "linux" => println!("   🐧 Linux:   {}", linux_cmd),
            "macos" => println!("   🍎 macOS:   {}", mac_cmd),
            "windows" => println!("   🪟 Windows: {}", win_cmd),
            _ => println!("   Установите {} вручную с официального сайта", tool),
        }
        println!();
    };

    // Проверяем Node.js
    if let Some(required) = env.get("NODE_VERSION") {
        if let Some(actual) = get_system_version("node", &["-v"]) {
            if !compare_versions(required, &actual) {
                println!("⚠️ Node.js: MISMATCH (требуется {}, у вас {})", required, actual);
                print_fix("Node.js", os, "curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - && sudo apt install -y nodejs", "brew install node", "winget install OpenJS.NodeJS.LTS");
                issues_found = true;
            }
        } else {
            println!("❌ Node.js: NOT INSTALLED (требуется {})", required);
            print_fix("Node.js", os, "sudo apt update && sudo apt install nodejs npm", "brew install node", "winget install OpenJS.NodeJS.LTS");
            issues_found = true;
        }
    }

    // Проверяем Go
    if let Some(required) = env.get("GO_VERSION") {
        if let Some(actual) = get_system_version("go", &["version"]) {
            let raw_version = actual.split_whitespace().nth(2).unwrap_or(&actual);
            let version = raw_version.trim_start_matches("go").split('-').next().unwrap_or(raw_version);
            if !compare_versions(required, version) {
                println!("⚠️ Go: MISMATCH (требуется {}, у вас {})", required, version);
                print_fix("Go", os, "sudo apt install golang-go", "brew install go", "winget install GoLang.Go");
                issues_found = true;
            }
        } else {
            println!("❌ Go: NOT INSTALLED (требуется {})", required);
            print_fix("Go", os, "sudo apt install golang-go", "brew install go", "winget install GoLang.Go");
            issues_found = true;
        }
    }

    // Проверяем Python
    if env.contains_key("PYTHON_DEPS") {
        let python_cmd = if get_system_version("python3", &["--version"]).is_some() { "python3" } else { "python" };
        if get_system_version(python_cmd, &["--version"]).is_none() {
            println!("❌ Python: NOT INSTALLED");
            print_fix("Python", os, "sudo apt install python3 python3-pip", "brew install python", "winget install Python.Python.3.12");
            issues_found = true;
        }
    }

    // Проверяем Rust
    if env.contains_key("RUST_PROJECT") {
        if get_system_version("rustc", &["--version"]).is_none() {
            println!("❌ Rust: NOT INSTALLED");
            print_fix("Rust", os, "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh", "brew install rust", "winget install Rustlang.Rustup");
            issues_found = true;
        }
    }

    if !issues_found {
        println!("✅ Проблем не обнаружено! Ваше окружение полностью соответствует требованиям проекта. 🚀");
    } else {
        println!("💡 После установки зависимостей запустите 'zeroenv check', чтобы убедиться, что всё работает.");
    }
}
