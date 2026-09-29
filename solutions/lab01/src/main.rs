fn main() {
    let order_id: u32 = 1001;
    let batch_volume_m3: f64 = 2.5;
    let cement_kg_per_m3: f64 = 320.0;
    let mut produced_m3: f64 = 0.0;

    let cement_target_kg = batch_volume_m3 * cement_kg_per_m3;
    produced_m3 += batch_volume_m3;

    println!("Order: {order_id}");
    println!("Cement target: {cement_target_kg:.1} kg");
    println!("Produced: {produced_m3:.1} m3");

    let label = "  Batch A  ";
    let label = label.trim();
    println!("Label: '{label}'");
}
