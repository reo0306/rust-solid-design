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
    fn create_credit_card(owner: String) -> CreditCard;
    fn register_credit_card(credit_card: CreditCard);
}

//struct CreditCardDatabase {
    //credit_card_database: Vec<&CreditCard>,
//}

struct PlatinumCreditCardFactory;

impl PlatinumCreditCardFactory {
    fn new() -> Self { Self }

    fn create(&self, owner: &String) -> CreditCard {
        credit_card = self.create_credit_card(owner);
        self.register_credit_card(credit_card);

        credit_card
    }
}

impl CreditCardFactory for PlatinumCreditCardFactory {
    fn create_credit_card(&self, owner: String) -> CreditCard {
        Platinum { owner }
    }

    fn register_credit_card(&self, credit_card_database: Vec<CreditCard>, credit_card: &CreditCard) {
        credit_card_database.push(credit_card)
    }
}

struct GoldCreditCardFactory;

impl GoldCreditCardFactory {
    fn new() -> Self { Self }

    fn create(&self, owner: &String) -> CreditCard {
        credit_card = self.create_credit_card(owner);
        self.register_credit_card(credit_card);

        credit_card
    }
}

impl CreditCardFactory for GoldCreditCardFactory {
    fn create_credit_card(owner: String) -> CreditCard {
        Gold { owner }
    }

    fn register_credit_card(credit_card: &CreditCard) {
        credit_card_database.push(credit_card)
    }
}

pub struct FactoryMain;

impl FacadeMain {
    pub fn index() {
        let credit_card_database: Vec<&CreditCard> = Vec::new();

        platinum_credit_card_factory = PlatinumCreditCardFactory::new();
        platinum_card = platinum_credit_card_factory.create("Tanaka".to_string());
        println!("{}", platinum_card.get_card_type());

        gold_credit_card_factory = GoldCreditCardFactory::new();
        gold_card = gold_credit_card_factory.create("Suzuki".to_string());
        println!("{}", gold_card.get_card_type());

        println!("{}", credit_card_database);
    }
}