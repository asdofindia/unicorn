use std::env;

const VERSION: &'static str = "0.1.0";

fn help() {
    println!("unicorn :: Unified Communications Over Real-time Networks");
    println!("Version: v{}", VERSION);
    println!("\nAvailable commands:");
    println!(" --version - Show program version");
    println!(" --help    - Show this help message");
}

fn print_version() {
    println!("{}", VERSION);
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
