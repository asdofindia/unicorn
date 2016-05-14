extern crate unicorn;

#[macro_use]
extern crate log;

use unicorn::logger::CLILogger;
use std::env;

fn help() {
    info!("unicorn :: Unified Communications Over Real-time Networks");
    info!("Version: v{}", unicorn::get_version());
    info!("\nUsage: unicorn [command] [[args]]");
    info!("\nAvailable commands:");
    info!(" core - Run unicorn core service");
    info!(" gateway - Run gateway API service");
    info!("\nAdditional information:");
    info!(" --version - Show program version");
    info!(" --help    - Show this help message\n");
    info!("\nFor verbose output, set \"LOGLEVEL\" environment variable to one of the following values:");
    info!(" \"info\" (default), \"warn\", \"error\", \"debug\".");
    info!(" e.g.: $ LOGLEVEL=debug ./unicorn\n");
    debug!("For more information, see: https://muktakosh/c/unicorn");
    debug!("Help make unicorn better. Contribute at https://github.com/muktakosh/unicorn");
}

fn print_version() {
    info!("{}", unicorn::get_version());
}

fn main() {
    match env::var("LOGLEVEL") {
        Ok(ref loglevel) => CLILogger::init(loglevel).unwrap(),
        Err(_) => CLILogger::init("info").unwrap()
    };

    if let Some(arg) = env::args().nth(1) {
        match arg.as_ref() {
            "--version" => print_version(),
            "core" => unicorn::core::run(),
            "gateway" => unicorn::gateway::run(),
            _ => help(),
        }
    } else {
        help()
    }
}
