extern crate unicorn;
extern crate core;
extern crate gateway;

use std::env;

fn help() {
    println!("unicorn :: Unified Communications Over Real-time Networks");
    println!("Version: v{}", unicorn::VERSION);
    println!("\nUsage: unicorn [command] [[args]]");
    println!("\nAvailable commands:");
    println!(" core - Run unicorn core service");
    println!(" gateway - Run gateway API service");
    println!("\nAdditional information:");
    println!(" --version - Show program version");
    println!(" --help    - Show this help message");
}

fn print_version() {
    println!("{}", unicorn::VERSION);
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        match arg.as_ref() {
            "--version" => print_version(),
            "core" => core::run(),
            "gateway" => gateway::run(),
            _ => help(),
        }
    } else {
        help()
    }
}
