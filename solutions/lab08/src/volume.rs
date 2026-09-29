pub fn parse_volume(text: &str) -> Result<f64, String> {
    let volume = text
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("Invalid number: {text}"))?;

    if !volume.is_finite() || volume <= 0.0 {
        return Err(String::from("Volume must be finite and greater than zero"));
    }
    Ok(volume)
}


#[cfg(test)]
mod tests {
    use super::parse_volume;

    #[test]
    fn accepts_trimmed_positive_input() {
        let value = parse_volume(" 2.5 ").expect("valid fixture");
        assert!((value - 2.5).abs() < 1e-9);
    }

    #[test]
    fn rejects_non_numeric_input() {
        assert!(parse_volume("oops").is_err());
    }

    #[test]
    fn rejects_non_positive_and_non_finite_input() {
        for input in ["0", "-1", "NaN", "inf"] {
            assert!(parse_volume(input).is_err(), "input was {input}");
        }
    }
}
