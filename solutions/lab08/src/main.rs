use rust_lab::volume::parse_volume;
use std::fmt::Display;

fn print_value<T: Display>(label: &str, value: T) {
    println!("{label}: {value}");
}

fn main() {
    match parse_volume("2.5") {
        Ok(volume) => print_value("Volume", volume),
        Err(error) => eprintln!("Error: {error}"),
    }
    print_value("Status", "ready");
}
