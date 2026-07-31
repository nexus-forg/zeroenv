use std::path::Path;
use std::fs::{self, OpenOptions};
use std::io::Write;
use serde::Deserialize;

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
