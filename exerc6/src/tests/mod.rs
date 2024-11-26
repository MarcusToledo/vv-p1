use super::*;
use shipping::calculate_shipping_cost;

    #[test]
    fn test_local_shipping_cost() {
        assert_eq!(calculate_shipping_cost(3.0, "LOCAL"), 10.0);
        assert_eq!(calculate_shipping_cost(10.0, "LOCAL"), 15.0);
        assert_eq!(calculate_shipping_cost(25.0, "LOCAL"), 50.0);
    }

    #[test]
    fn test_international_shipping_cost() {
        assert_eq!(calculate_shipping_cost(3.0, "OUTRO"), 20.0);
        assert_eq!(calculate_shipping_cost(10.0, "OUTRO"), 30.0);
        assert_eq!(calculate_shipping_cost(25.0, "OUTRO"), 50.0);
    }
