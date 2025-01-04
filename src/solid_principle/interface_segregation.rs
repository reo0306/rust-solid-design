trait Vehicel {
    fn new(name: String, color: String) -> Self;
}

trait Movable {
    fn start(&self);
    fn stop(&self);
}

trait Flyable {
    fn fly(&self);
}

pub struct Airplane {
    name: String,
    color: String,
}

impl Vehicel for Airplane {
    fn new(name: String, color: String) -> Self {
        Self { name, color, }
    }
}

impl Movable for Airplane {
    fn start(&self) {
        println!("start!");
    }

    fn stop(&self) {
        println!("stop!");
    }
}

impl Flyable for Airplane {
    fn fly(&self) {
        println!("fly!");
    }
}

pub struct Car {
    name: String,
    color: String,
}

impl Vehicel for Car {
    fn new(name: String, color: String) -> Self {
        Self { name, color, }
    }
}

impl Movable for Car {
    fn start(&self) {
        println!("start!");
    }

    fn stop(&self) {
        println!("stop!");
    }
}

pub struct InterfaceMain;

impl InterfaceMain {
    pub fn index() {
        let v1 = Airplane::new("AirBus".to_string(), "white".to_string());
        let v2 = Car::new("Prius".to_string(), "black".to_string());

        println!("{}, {}", v1.name, v1.color);
        v1.fly();
        println!("{}, {}", v2.name, v2.color);
        v2.start();
    }
}