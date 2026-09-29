#[derive(Debug, PartialEq)]
enum Reading {
    Normal,
    High,
    Invalid,
}

fn classify(value: f64) -> Reading {
    if !value.is_finite() || !(0.0..=100.0).contains(&value) {
        Reading::Invalid
    } else if value >= 80.0 {
        Reading::High
    } else {
        Reading::Normal
    }
}

fn main() {
    for value in [42.0, 80.0, -1.0, f64::NAN] {
        println!("{value}: {:?}", classify(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_threshold_boundaries() {
        assert_eq!(classify(0.0), Reading::Normal);
        assert_eq!(classify(79.9), Reading::Normal);
        assert_eq!(classify(80.0), Reading::High);
        assert_eq!(classify(100.0), Reading::High);
    }

    #[test]
    fn rejects_invalid_readings() {
        for value in [-0.1, 100.1, f64::NAN, f64::INFINITY] {
            assert_eq!(classify(value), Reading::Invalid);
        }
    }
}
