use clap::{Arg, Command};
use serde_json::json;

fn main() {
    let matches = Command::new("lsblk")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Rust CLI example using clap v4")
        .arg(
            Arg::new("device")
                .short('d')
                .long("device")
                .value_name("DEVICE")
                .help("Device name to query"),
        )
        .get_matches();

    let device = matches
        .get_one::<String>("device")
        .map(|s| s.as_str())
        .unwrap_or("sda");

    let result = json!({
        "device": device,
        "size": "500G",
        "type": "SSD"
    });

    println!("{}", result.to_string());
}
