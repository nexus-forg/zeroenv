use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::Write;
use serde::Deserialize;
use std::process::Command;
use std::collections::HashMap;

#[derive(Deserialize)]
struct PackageJson {
    engines: Option<Engines>,
}

#[derive(Deserialize)]
struct Engines {
    node: Option<String>,
}

fn save_detected_env(lang: &str, version: &str) -> std::io::Result<()> {
    let config_line = format!("{}={}\n", lang, version);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".zeroenv")?;
    file.write_all(config_line.as_bytes())?;
    Ok(())
}

fn parse_package_json() -> Option<String> {
    let content = fs::read_to_string("package.json").ok()?;
    let pkg: PackageJson = serde_json::from_str(&content).ok()?;
    pkg.engines?.node
}

fn parse_requirements_txt() -> Option<String> {
    let content = fs::read_to_string("requirements.txt").ok()?;
    let mut versions = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if versions.len() < 3 {
            versions.push(line.to_string());
        }
    }
    
    if versions.is_empty() { None } else { Some(versions.join(", ")) }
}

fn parse_go_mod() -> Option<String> {
    let content = fs::read_to_string("go.mod").ok()?;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("go ") {
            return Some(line[3..].to_string());
        }
    }
    None
}

fn get_system_version(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .ok()?;
    
    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Some(version)
    } else {
        None
    }
}

fn parse_zeroenv() -> HashMap<String, String> {
    let mut env = HashMap::new();
    
    if let Ok(content) = std::fs::read_to_string(".zeroenv") {
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                env.insert(key.to_string(), value.to_string());
            }
        }
    }
    
    env
}

fn compare_versions(required: &str, actual: &str) -> bool {
    // Убираем префиксы типа ">=", "v", "go"
    let required_clean = required.trim_start_matches(">=").trim_start_matches('v');
    let actual_clean = actual.trim_start_matches('v');
    
    // Извлекаем мажорную версию (число до первой точки)
    let required_major = required_clean.split('.').next();
    let actual_major = actual_clean.split('.').next();
    
    if let (Some(req), Some(act)) = (required_major, actual_major) {
        if let (Ok(req_num), Ok(act_num)) = (req.parse::<u32>(), act.parse::<u32>()) {
            return act_num >= req_num;
        }
    }
    
    false
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        println!("zeroenv - Zero-configuration development environment manager");
        println!("\nUsage:");
        println!("  zeroenv init     Scan project directory and generate .zeroenv config");
        println!("  zeroenv status   Display current project environment configuration");
        return;
    }

    match args[1].as_str() {

	"check" => {
    let env = parse_zeroenv();
    
    if env.is_empty() {
        println!("❌ Файл .zeroenv не найден или пуст. Запустите сначала 'zeroenv init'");
        return;
    }
    
    println!("🔍 Проверяю окружение...\n");
    
    // Проверяем Rust
    if env.contains_key("RUST_PROJECT") {
        if let Some(actual) = get_system_version("rustc", &["--version"]) {
        // rustc --version выводит "rustc 1.75.0 (82e1608df 2023-12-21)"
        // Нужно извлечь "1.75.0"
            let version = actual.split_whitespace().nth(1).unwrap_or(&actual);
            println!("✅ Rust: INSTALLED (rustc {})", version);
        } else {
            println!("❌ Rust: NOT INSTALLED (project requires Rust toolchain)");
        }
    }

    // Проверяем Python
    if env.contains_key("PYTHON_DEPS") {
    // Пробуем сначала python3, потом python
        let python_cmd = if get_system_version("python3", &["--version"]).is_some() {
            "python3"
        } else {
            "python"
        };
    
        if let Some(actual) = get_system_version(python_cmd, &["--version"]) {
        // python --version выводит "Python 3.11.5", нужно извлечь "3.11.5"
            let version = actual.split_whitespace().nth(1).unwrap_or(&actual);
            println!("✅ Python: INSTALLED (version {})", version);
            println!("   📦 Dependencies from .zeroenv: {}", env.get("PYTHON_DEPS").unwrap());
        } else {
            println!("❌ Python: NOT INSTALLED (project has dependencies)");
        }
    }

    // Проверяем Node.js
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
    
    // Проверяем Go
    if let Some(required) = env.get("GO_VERSION") {
        if let Some(actual) = get_system_version("go", &["version"]) {
            let raw_version = actual.split_whitespace().nth(2).unwrap_or(&actual);
            let version = raw_version
                .trim_start_matches("go")  // Убираем префикс "go"
                .split('-')                // Разделяем по "-"
                .next()                    // Берём первую часть
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

        "init" => {
            println!("Scanning project directory...");
            let _ = fs::remove_file(".zeroenv");

            if Path::new("package.json").exists() {
                println!("Detected: Node.js project");
                let version = parse_package_json().unwrap_or_else(|| "latest".to_string());
                println!("  Required Node.js: {}", version);
                let _ = save_detected_env("NODE_VERSION", &version);
            }
            
            if Path::new("requirements.txt").exists() {
                println!("Detected: Python project");
                if let Some(deps) = parse_requirements_txt() {
                    println!("  Dependencies: {}", deps);
                    let _ = save_detected_env("PYTHON_DEPS", &deps);
                }
            }
            
            if Path::new("go.mod").exists() {
                println!("Detected: Go project");
                let version = parse_go_mod().unwrap_or_else(|| "latest".to_string());
                println!("  Required Go: {}", version);
                let _ = save_detected_env("GO_VERSION", &version);
            }
            
            if Path::new("Cargo.toml").exists() {
                println!("Detected: Rust project");
                let _ = save_detected_env("RUST_PROJECT", "true");
            }

            println!("Configuration successfully saved to .zeroenv");
        }
        "status" => {
            if Path::new(".zeroenv").exists() {
                println!("Current project environment (.zeroenv):");
                if let Ok(content) = fs::read_to_string(".zeroenv") {
                    print!("{}", content);
                }
            } else {
                eprintln!("Error: .zeroenv not found. Run 'zeroenv init' first.");
            }
        }
        _ => {
            eprintln!("Error: Unknown command '{}'. Use '--help' for usage information.", args[1]);
        }
    }
}
