pub fn calculate_shipping_cost(weight: f64 , destination: &str) -> f64 {
    let mut cost = 0.0;

    if destination == "LOCAL" {
        match weight {
            0.0..=5.0 => cost = 10.0,
            5.0..=20.0 => cost = 15.0,
            _ => cost = 50.0,
        }
    } else if destination == "OUTRO" {
        match weight {
            0.0..=5.0 => cost = 20.0,
            5.0..=20.0 => cost = 30.0,
            _ => cost = 50.0,
        }
    }

    cost
}