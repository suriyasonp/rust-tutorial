use std::process::Command;

#[test]
fn cli_prints_summary_for_fixture() {
    let output = Command::new(env!("CARGO_BIN_EXE_batch_cli"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("data/orders.txt")
        .output()
        .expect("start CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let normalized = stdout.replace("\r\n", "\n");
    assert_eq!(normalized, "Orders: 3\nTotal volume: 8.00 m3\n");
}

#[test]
fn cli_fails_without_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_batch_cli"))
        .output()
        .expect("start CLI");
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr).unwrap().contains("Usage:"));
}
