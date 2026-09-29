fn cement_target(volume_m3: f64, dosage_kg_per_m3: f64) -> f64 {
    volume_m3 * dosage_kg_per_m3
}

fn classify(volume_m3: f64) -> &'static str {
    if volume_m3 <= 0.0 {
        "invalid"
    } else if volume_m3 <= 3.0 {
        "small"
    } else {
        "large"
    }
}

fn main() {
    let volumes = [2.5, 4.0, 0.0];
    for volume in volumes {
        println!("{volume:.1} m3: {}", classify(volume));
    }
    println!("Target: {:.1} kg", cement_target(2.5, 320.0));
}
