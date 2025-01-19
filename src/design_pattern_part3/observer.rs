trait Observer {
    fn update(&self, name: &str);
}

struct StoreObserver;

impl StoreObserver {
    fn new() -> Self { Self }
}

impl Observer for StoreObserver {
    fn update(&self, name: &str) {
        println!("{}が入荷されました。仕入れが可能です。", name);
    }
}

struct PersonalObserver;

impl PersonalObserver {
    fn new() -> Self { Self }
}

impl Observer for PersonalObserver {
    fn update(&self, name: &str) {
        println!("{}が入荷されました。購入が可能です。", name);
    }
}

trait ItemSubject {
    fn restock(&self);
}

struct TVGameSubject {
    name: String,
    observers: Vec<Box<dyn Observer>>,
}

impl TVGameSubject {
    fn new(name: String) -> Self {
        Self {
            name,
            observers: Vec::new(),
        }
    }

    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    /*fn detach(&mut self, observer: Box<dyn Observer>) {
        self.observers.remove(observer);
    }*/

    fn notify(&self) {
        for obs in &self.observers {
            obs.update(&self.name);
        }
    }
}

impl ItemSubject for TVGameSubject {
    fn restock(&self) {
        println!("TVゲームの入荷");
        self.notify();
    }
}

pub struct ObserverMain;

impl ObserverMain {
    pub fn index() {
        let store = StoreObserver::new();
        let person = PersonalObserver::new();
        let mut tv_game = TVGameSubject::new("New RPG Game".to_string());

        tv_game.attach(Box::new(store));
        tv_game.attach(Box::new(person));
        tv_game.restock();
    }
}