#[derive(Debug)]
pub struct InvalidProductException;

impl std::fmt::Display for InvalidProductException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid product")
    }
}

impl std::error::Error for InvalidProductException {}

pub struct Produto {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

pub fn calculate_invoice(products: Vec<Produto>, discount: f64) -> Result<f64, InvalidProductException> {
    let mut total_price: f64 = 0.0;

    for produto in &products {
        if produto.quantity < 0 || produto.price < 0.0 {
            return Err(InvalidProductException);
        }

        total_price += produto.price * produto.quantity as f64;
    }

    // Aplicando o desconto
    let discount_value = (total_price * discount / 100.0).round();
    let mut final_price = total_price - discount_value;

    if total_price > 1000.0 {
        final_price -= 100.0;
    }

    final_price = (final_price * 100.0).round() / 100.0;

    if final_price < 0.0 {
        final_price = 0.0;
    }//Corrigir caso de teste 8

    Ok(final_price)
}