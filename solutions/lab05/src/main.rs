fn main() {
    let mut volumes = vec![2.5, 4.0, 1.5];
    volumes.push(3.0);

    let total: f64 = volumes.iter().sum();
    let large: Vec<f64> = volumes
        .iter()
        .copied()
        .filter(|volume| *volume > 3.0)
        .collect();

    println!("Count: {}", volumes.len());
    println!("Total: {total:.1} m3");
    println!("Large: {large:?}");

    match volumes.get(10) {
        Some(volume) => println!("Found: {volume}"),
        None => println!("No order at index 10"),
    }
}
