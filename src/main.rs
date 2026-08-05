use std::env;

mod config;
mod scanner;
mod checker;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        println!("zeroenv - Zero-configuration development environment manager");
        println!("\nUsage:");
        println!("  zeroenv init     Scan project directory and generate .zeroenv config");
        println!("  zeroenv check    Validate system against .zeroenv requirements");
        println!("  zeroenv doctor   Diagnose issues and suggest installation commands");
        println!("  zeroenv status   Display current project environment configuration");
        return;
    }

    match args[1].as_str() {
        "init" => scanner::run_init(),
        "check" => checker::run_check(),
        "doctor" => checker::run_doctor(),  // <-- ВОТ ЭТА СТРОКА БЫЛА ПРОПУЩЕНА
        "status" => checker::run_status(),
        _ => {
            eprintln!("Error: Unknown command '{}'. Use '--help' for usage information.", args[1]);
        }
    }
}
