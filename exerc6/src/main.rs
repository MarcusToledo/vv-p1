mod shipping;

#[cfg(test)]
mod tests;
fn main() {
    let weight = 5.0;
    let destination = "LOCAL";
    let cost = shipping::calculate_shipping_cost(weight, destination);
    println!("The shipping cost is: R${}", cost);
}



