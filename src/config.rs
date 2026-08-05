use std::fs::{self, OpenOptions};
use std::io::Write;
use std::collections::HashMap;

pub fn save_detected_env(lang: &str, version: &str) -> std::io::Result<()> {
    let config_line = format!("{}={}\n", lang, version);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".zeroenv")?;
    file.write_all(config_line.as_bytes())?;
    Ok(())
}

pub fn parse_zeroenv() -> HashMap<String, String> {
    let mut env = HashMap::new();
    if let Ok(content) = fs::read_to_string(".zeroenv") {
        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                env.insert(key.to_string(), value.to_string());
            }
        }
    }
    env
}
