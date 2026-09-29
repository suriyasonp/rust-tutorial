fn print_order(name: &str) {
    println!("Order: {name}");
}

fn add_suffix(name: &mut String) {
    name.push_str("-READY");
}

fn main() {
    let mut order = String::from("Order-1001");
    print_order(&order);
    add_suffix(&mut order);
    print_order(&order);
    println!("Still owned by main: {order}");
}
