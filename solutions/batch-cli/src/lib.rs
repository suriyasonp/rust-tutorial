#[derive(Debug)]
pub struct Summary {
    pub count: usize,
    pub total_m3: f64,
}

pub fn parse_volume(text: &str) -> Result<f64, String> {
    let volume = text.trim().parse::<f64>()
        .map_err(|_| format!("Invalid number: {text}"))?;
    if !volume.is_finite() || volume <= 0.0 {
        return Err(String::from("Volume must be finite and greater than zero"));
    }
    Ok(volume)
}

pub fn parse_orders(contents: &str) -> Result<Vec<f64>, String> {
    let mut orders = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let volume = parse_volume(line)
            .map_err(|error| format!("Line {}: {error}", index + 1))?;
        orders.push(volume);
    }
    if orders.is_empty() {
        return Err(String::from("No orders found"));
    }
    Ok(orders)
}

pub fn summarize(orders: &[f64]) -> Result<Summary, String> {
    if orders.is_empty() {
        return Err(String::from("No orders found"));
    }
    let mut total_m3 = 0.0;
    for &volume in orders {
        if !volume.is_finite() || volume <= 0.0 {
            return Err(String::from("Invalid order volume"));
        }
        total_m3 += volume;
        if !total_m3.is_finite() {
            return Err(String::from("Total volume exceeds numeric range"));
        }
    }
    Ok(Summary { count: orders.len(), total_m3 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_valid_orders() {
        let orders = parse_orders("2.5\n4.0\n1.5\n").unwrap();
        let summary = summarize(&orders).unwrap();
        assert_eq!(summary.count, 3);
        assert!((summary.total_m3 - 8.0).abs() < 1e-9);
    }

    #[test]
    fn supports_crlf_and_blank_lines() {
        assert_eq!(parse_orders("2.5\r\n\r\n 1.5 \r\n").unwrap(), vec![2.5, 1.5]);
    }

    #[test]
    fn preserves_physical_error_line_number() {
        let error = parse_orders("2.5\n\noops\n").unwrap_err();
        assert_eq!(error, "Line 3: Invalid number: oops");
    }

    #[test]
    fn rejects_empty_input() {
        for input in ["", " \n\r\n"] {
            assert_eq!(parse_orders(input).unwrap_err(), "No orders found");
        }
    }

    #[test]
    fn rejects_invalid_quantities() {
        for input in ["0", "-2", "NaN", "inf", "oops"] {
            assert!(parse_orders(input).is_err());
        }
    }

    #[test]
    fn rejects_overflowing_total() {
        assert!(summarize(&[f64::MAX, f64::MAX]).is_err());
    }

    #[test]
    fn rejects_invalid_direct_summary_input() {
        assert!(summarize(&[]).is_err());
        assert!(summarize(&[0.0]).is_err());
        assert!(summarize(&[f64::NAN]).is_err());
    }
}
