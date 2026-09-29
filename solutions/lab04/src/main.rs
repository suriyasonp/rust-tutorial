#[derive(Debug)]
enum Status {
    Planned,
    Completed,
}

#[derive(Debug)]
struct Order {
    id: u32,
    volume_m3: f64,
    status: Status,
}

impl Order {
    fn new(id: u32, volume_m3: f64) -> Self {
        Self { id, volume_m3, status: Status::Planned }
    }

    fn complete(&mut self) {
        self.status = Status::Completed;
    }

    fn status_label(&self) -> &'static str {
        match &self.status {
            Status::Planned => "planned",
            Status::Completed => "completed",
        }
    }
}

fn main() {
    let mut order = Order::new(1001, 2.5);
    println!("{}: {:.1} m3, {}", order.id, order.volume_m3, order.status_label());
    order.complete();
    println!("{}: {}", order.id, order.status_label());
}
