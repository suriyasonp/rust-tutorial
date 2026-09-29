use batch_cli::{parse_orders, summarize};
use std::{env, fs, process::ExitCode};

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let path = args.next()
        .ok_or_else(|| String::from("Usage: batch_cli <orders-file>"))?;
    if args.next().is_some() {
        return Err(String::from("Usage: batch_cli <orders-file>"));
    }

    let contents = fs::read_to_string(&path)
        .map_err(|error| format!("Could not read {:?}: {error}", path))?;
    let orders = parse_orders(&contents)?;
    let summary = summarize(&orders)?;

    println!("Orders: {}", summary.count);
    println!("Total volume: {:.2} m3", summary.total_m3);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
