use std::env;
use std::fs;
use std::io::Error;

/// Reads an environment variable via std::env::var.
fn main() -> Result<(), Error> {
    // read `config_path` from the environment variable `CONFIG`
    // If `CONFIG` isn't set, fall back to a default config path. 
    let config_path = env::var("CONFIG")
        .unwrap_or("/etc/myapp/config".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}