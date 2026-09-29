#[derive(Debug, Default, PartialEq)]
struct Counts {
    info: usize,
    warn: usize,
    error: usize,
}

fn summarize(text: &str) -> Result<Counts, String> {
    let mut counts = Counts::default();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (level, message) = line.split_once('|')
            .ok_or_else(|| format!("Line {}: missing separator", index + 1))?;
        if message.trim().is_empty() {
            return Err(format!("Line {}: empty message", index + 1));
        }
        match level.trim() {
            "INFO" => counts.info += 1,
            "WARN" => counts.warn += 1,
            "ERROR" => counts.error += 1,
            _ => return Err(format!("Line {}: unknown level", index + 1)),
        }
    }
    Ok(counts)
}

fn main() {
    let input = include_str!("../data/service.txt");
    match summarize(input) {
        Ok(counts) => println!("INFO={} WARN={} ERROR={}", counts.info, counts.warn, counts.error),
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_fixture() {
        let result = summarize(include_str!("../data/service.txt")).unwrap();
        assert_eq!(result, Counts { info: 2, warn: 1, error: 1 });
    }

    #[test]
    fn rejects_malformed_records() {
        for input in ["INFO", "INFO| ", "DEBUG|message"] {
            assert!(summarize(input).is_err());
        }
    }

    #[test]
    fn accepts_empty_logs_and_embedded_separators() {
        assert_eq!(summarize("\n").unwrap(), Counts::default());
        assert_eq!(summarize("INFO|left|right").unwrap().info, 1);
    }
}
