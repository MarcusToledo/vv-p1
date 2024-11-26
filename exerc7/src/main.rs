mod invoice;
use invoice::Produto;

#[cfg(test)]
mod tests;

fn main() {
    let products = vec![
        Produto {
            name: "Product A".to_string(),
            price: 200.0,
            quantity: 3,
        },
        Produto {
            name: "Product B".to_string(),
            price: 150.0,
            quantity: 4,
        },
    ];

    let discount = 10.0;

    match invoice::calculate_invoice(products, discount) {
        Ok(final_price) => println!("Final price: R$ {:.2}", final_price),
        Err(e) => println!("Error: {}", e),
    }
}