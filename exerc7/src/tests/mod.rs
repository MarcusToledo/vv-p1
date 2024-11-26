use super::*;
use invoice::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 50.0,
            quantity: 5,
        }];
        let result = calculate_invoice(products, 10.0);
        assert_eq!(result.unwrap(), 225.00);
    }

    #[test]
    fn test_case_2() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 500.0,
            quantity: 3,
        }];
        let result = calculate_invoice(products, 20.0);
        assert_eq!(result.unwrap(), 1100.00);
    }

    #[test]
    fn test_case_3() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 250.0,
            quantity: 5,
        }];
        let result = calculate_invoice(products, 0.0);
        assert_eq!(result.unwrap(), 1150.00);
    }

    #[test]
    fn test_case_4() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 100.0,
            quantity: -1,
        }];
        let result = calculate_invoice(products, 10.0);
        assert!(matches!(result, Err(InvalidProductException)));
    }

    #[test]
    fn test_case_5() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: -50.0,
            quantity: 2,
        }];
        let result = calculate_invoice(products, 10.0);
        assert!(matches!(result, Err(InvalidProductException)));
    }

    #[test]
    fn test_case_6() {
        let products = vec![];
        let result = calculate_invoice(products, 10.0);
        assert_eq!(result.unwrap(), 0.00);
    }

    #[test]
    fn test_case_7() {
        let products = vec![
            Produto {
                name: "Produto A".to_string(),
                price: 200.0,
                quantity: 2,
            },
            Produto {
                name: "Produto B".to_string(),
                price: 100.0,
                quantity: 3,
            },
        ];
        let result = calculate_invoice(products, 15.0);
        assert_eq!(result.unwrap(), 595.00);
    }

    #[test]
    fn test_case_8() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 1200.0,
            quantity: 1,
        }];
        let result = calculate_invoice(products, 100.0);
        assert_eq!(result.unwrap(), 0.00);
    }

    #[test]
    fn test_case_9() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 300.0,
            quantity: 2,
        }];
        let result = calculate_invoice(products, 0.0);
        assert_eq!(result.unwrap(), 600.00);
    }

    #[test]
    fn test_case_10() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 500.0,
            quantity: 2,
        }];
        let result = calculate_invoice(products, 10.0);
        assert_eq!(result.unwrap(), 900.00);
    }

    #[test]
    fn test_case_11() {
        let products = vec![Produto {
            name: "Produto A".to_string(),
            price: 333.34,
            quantity: 3,
        }];
        let result = calculate_invoice(products, 10.0);
        assert_eq!(result.unwrap(), 800.02);
    }

    #[test]
    fn test_case_12() {
        let products = vec![
            Produto {
                name: "Produto A".to_string(),
                price: 300.0,
                quantity: 2,
            },
            Produto {
                name: "Produto B".to_string(),
                price: 500.0,
                quantity: 1,
            },
        ];
        let result = calculate_invoice(products, 25.0);
        assert_eq!(result.unwrap(), 725.0); //Ajustar nos casos de teste fiz o calc errado
    }
}