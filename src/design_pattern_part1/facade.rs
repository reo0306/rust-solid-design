struct Product;

impl Product {
    fn get_product(name: &String) {
        println!("{}を取得しました", name);
    }
}

struct Payment;

impl Payment {
    fn make_payment(name: &String) {
        println!("{}の支払いが完了しました", name);
    }
}

struct Invoice;

impl Invoice {
    fn send_invoice(name: &String) {
        println!("{}の請求が送信されました", name);
    }
}

struct Order;

impl Order {
    fn place_order(name: &String) {
        println!("注文開始");

        Product::get_product(&name);
        Payment::make_payment(&name);
        Invoice::send_invoice(&name);

        println!("注文が正常に完了しました");
    }
}

pub struct FacadeMain;

impl FacadeMain {
    pub fn index() {
        let name = "デザインパターン本".to_string();
        Order::place_order(&name);
    }
}