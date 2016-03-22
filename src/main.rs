extern crate unicorn;

use std::env;

fn help() {
    println!("unicorn :: Unified Communications Over Real-time Networks");
    println!("Version: v{}", unicorn::VERSION);
    println!("\nAvailable commands:");
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
            _ => help(),
        }
    } else {
        help()
    }
}
