mod config;
mod util;
mod server;
mod message;
mod net;
mod behavior;
mod data;

use std::process;

fn main() {

    // Parse configuration file
    let config = config::get_config().unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        process::exit(1)
    });

    // Configure logging
    config::configure_logging(config.log_file.clone(), config.log_level.clone()).unwrap_or_else(|err| {
        eprintln!("Error with setting logging configuration: {}", err);
        process::exit(1)
    });

    // Run node
    server::start(config);
}

#[cfg(test)]
mod tests{
    // TODO
}