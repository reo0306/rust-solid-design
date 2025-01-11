use std::collections::HashMap;

trait PaymentStrategy {
    fn pay(&self, amount: i64);
}

struct CreditCardPaymentStrategy;

impl CreditCardPaymentStrategy {
    fn new() -> Self { Self { } }
}

impl PaymentStrategy for CreditCardPaymentStrategy {
    fn pay(&self, amount: i64) {
        println!("クレジットカードで{}円の支払い", amount);
    }
}

struct CashPaymentStrategy;

impl CashPaymentStrategy {
    fn new() -> Self { Self { } }
}

impl PaymentStrategy for CashPaymentStrategy {
    fn pay(&self, amount: i64) {
        println!("現金で{}円の支払い", amount);
    }
}

struct ShoppingCart {
    total: i64,
    items: HashMap<String, i64>,
}

impl ShoppingCart {
    fn new() -> Self {
        Self {
            total: 0,
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: String, price: i64) {
        self.total += price;
        self.items.insert(item, price);
    }

    fn pay(&self, payment_strategy: impl PaymentStrategy) {
        payment_strategy.pay(self.total)
    }
}

pub struct StrategyMain;

impl StrategyMain {
    pub fn index() {
        let mut cart = ShoppingCart::new();
        cart.add_item("item1".to_string(), 500);
        cart.add_item("item2".to_string(), 1000);

        let payment_strategy1 = CreditCardPaymentStrategy::new();
        cart.pay(payment_strategy1);

        let payment_strategy2 = CashPaymentStrategy::new();
        cart.pay(payment_strategy2);
    }
}