trait IEmployee {
    fn new(name: String) -> Self;
    fn get_bouns(&self, base: i64) -> f64;
}

pub struct JuniorEmployee {
    name: String,
}

impl IEmployee for JuniorEmployee {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn get_bouns(&self, base: i64) -> f64 {
        let bonus = base as f64 * 1.1;

        bonus.floor()
    }
}

pub struct MiddleEmployee {
    name: String,
}

impl IEmployee for MiddleEmployee {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn get_bouns(&self, base: i64) -> f64 {
        let bonus = base as f64 * 1.5;

        bonus.floor()
    }
}

pub struct SeniorEmployee {
    name: String,
}

impl IEmployee for SeniorEmployee {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn get_bouns(&self, base: i64) -> f64 {
        let bonus = (base * 2) as f64;

        bonus.floor()
    }
}

pub struct ExpertEmployee {
    name: String,
}

impl IEmployee for ExpertEmployee {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn get_bouns(&self, base: i64) -> f64 {
        let bonus = (base * 3) as f64;

        bonus.floor()
    }
}

pub struct OpenMain;

impl OpenMain {
    pub fn index() {
        let emp1 = JuniorEmployee::new("Yamada".to_string());
        let emp2 = MiddleEmployee::new("Suzuki".to_string());
        let emp3 = SeniorEmployee::new("Tanaka".to_string());
        let emp4 = ExpertEmployee::new("Bob".to_string());

        let base: i64 = 100;
        println!("{}, {}", emp1.name, emp1.get_bouns(base));
        println!("{}, {}", emp2.name, emp2.get_bouns(base));
        println!("{}, {}", emp3.name, emp3.get_bouns(base));
        println!("{}, {}", emp4.name, emp4.get_bouns(base));
    }
}