use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Stamp {
    character: String,
}

impl Stamp {
    fn new(character: String) -> Self {
        Self { character }
    }

    fn print_char(&self) {
        println!{"{}", self.character};
    }
}

struct StampFactroy {
    pool: HashMap<String, Stamp>,
}

impl StampFactroy {
    fn new() -> Self {
        Self { pool: HashMap::new() }
    }

    fn get_stamp(&mut self, character: String) -> Stamp {
        if let Some(stamp) = self.pool.get(&character) {
            return stamp.clone();
        }

        let new_stamp = Stamp::new(character.clone());

        self.pool.insert(character.clone(), new_stamp.clone());

        new_stamp.clone()
    }

    fn get_pool(&self) -> &HashMap<String, Stamp> {
        &self.pool
    }
}

pub struct FlyWeightMain;

impl FlyWeightMain {
    pub fn index() {
        let mut factory = StampFactroy::new();

        let stamp1 = factory.get_stamp("し".to_string());
        let stamp2 = factory.get_stamp("ん".to_string());
        let stamp3 = factory.get_stamp("ぶ".to_string());
        let stamp4 = factory.get_stamp("ん".to_string());
        let stamp5 = factory.get_stamp("し".to_string());

        stamp1.print_char();
        stamp2.print_char();
        stamp3.print_char();
        stamp4.print_char();
        stamp5.print_char();

        println!("{:?}", factory.get_pool());
    }
}