trait CreditCard {
    fn get_card_type(&self) -> String;
    fn get_annual_charge(&self) -> i64;
}

struct Platinum {
    owner: String,
}

impl Platinum {
    fn new(owner: String) -> Self {
        Self { owner }
    }
}

impl CreditCard for Platinum {
    fn get_card_type(&self) -> String {
        "Platinum".to_string()
    }

    fn get_annual_charge(&self) -> i64 {
        30000
    }
}

struct Gold {
    owner: String,
}

impl Gold {
    fn new(owner: String) -> Self {
        Self { owner }
    }
}


impl CreditCard for Gold {
    fn get_card_type() -> String {
        "Gold".to_string()
    }

    fn get_annual_charge() -> i64 {
        10000
    }
}

trait CreditCardFactory {
    fn create_credit_card(owner: String) -> dyn CreditCard;
    fn register_credit_card(credit_card: dyn CreditCard);
}

struct PlatinumCreditCardFactory {
    credit_card_database: Vec<CreditCard>,
}

impl PlatinumCreditCardFactory {
    fn new() -> Self { Self }

    fn create(&self, owner: &String) -> dyn CreditCard {
        let credit_card = self.create_credit_card(owner);
        self.register_credit_card(credit_card);

        credit_card
    }
}

impl CreditCardFactory for PlatinumCreditCardFactory {
    fn create_credit_card(&self, owner: String) -> dyn CreditCard {
        Platinum { owner }
    }

    fn register_credit_card(&self, credit_card_database: Vec<dyn CreditCard>, credit_card: &dyn CreditCard) {
        self.credit_card_database.push(credit_card)
    }
}

struct GoldCreditCardFactory {
    credit_card_database: Vec<CreditCard>,
}

impl GoldCreditCardFactory {
    fn new() -> Self {
        Self {

        }
    }

    fn create(&self, owner: &String) -> dyn CreditCard {
        let credit_card = self.create_credit_card(owner);
        self.register_credit_card(credit_card);

        credit_card
    }
}

impl CreditCardFactory for GoldCreditCardFactory {
    fn create_credit_card(owner: String) -> dyn CreditCard {
        Gold { owner }
    }

    fn register_credit_card(credit_card: &dyn CreditCard) {
        self.credit_card_database.push(credit_card)
    }
}

pub struct FactoryMain;

impl FacadeMain {
    pub fn index() {
        let credit_card_database: Vec<&CreditCard> = Vec::new();

        let platinum_credit_card_factory = PlatinumCreditCardFactory::new();
        let platinum_card = platinum_credit_card_factory.create("Tanaka".to_string());
        println!("{}", platinum_card.get_card_type());

        let gold_credit_card_factory = GoldCreditCardFactory::new();
        let gold_card = gold_credit_card_factory.create("Suzuki".to_string());
        println!("{}", gold_card.get_card_type());

        println!("{}", credit_card_database);
    }
}