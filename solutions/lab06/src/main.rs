fn parse_volume(text: &str) -> Result<f64, String> {
    let volume = text
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("Invalid number: {text}"))?;

    if !volume.is_finite() || volume <= 0.0 {
        return Err(String::from("Volume must be finite and greater than zero"));
    }
    Ok(volume)
}

fn main() {
    for input in ["2.5", "oops", "0", "NaN"] {
        match parse_volume(input) {
            Ok(volume) => println!("Accepted: {volume:.1} m3"),
            Err(error) => println!("Rejected: {error}"),
        }
    }
}
