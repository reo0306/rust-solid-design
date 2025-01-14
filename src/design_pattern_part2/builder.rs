#[derive(Debug, Default, Clone)]
struct Computer {
    name: String,
    cpu: String,
    ram: i64,
}

impl Computer {
    fn new() -> Self {
        Default::default()
    }
}

trait ComputerBuilder {
    fn add_cpu(&mut self, cpu: String);
    fn add_ram(&mut self, ram: i64);
}

#[derive(Clone)]
struct DesktopBuilder {
    computer: Computer,
}

impl DesktopBuilder {
    fn new() -> Self {
        let mut computer = Computer::new();
        computer.name = "Desktop".to_string();

        Self { computer }
    }

    fn get_result(&self) -> Computer {
        self.computer.clone()
    }
}

impl ComputerBuilder for DesktopBuilder {
    fn add_cpu(&mut self, cpu: String) {
        self.computer.cpu = cpu;
    }

    fn add_ram(&mut self, ram: i64) {
        self.computer.ram = ram;
    }
}

#[derive(Clone)]
struct LaptopBuilder {
    computer: Computer,
}

impl LaptopBuilder {
    fn new() -> Self {
        let mut computer = Computer::new();
        computer.name = "Laptop".to_string();

        Self { computer }
    }

    fn get_result(&self) -> Computer {
        self.computer.clone()
    }
}

impl ComputerBuilder for LaptopBuilder {
    fn add_cpu(&mut self, cpu: String) {
        self.computer.cpu = cpu;
    }

    fn add_ram(&mut self, ram: i64) {
        self.computer.ram = ram;
    }
}

struct Director<T: ComputerBuilder> {
    builder: T,
}

impl<T: ComputerBuilder> Director<T> {
    fn new(builder: T) -> Self {
        Self { builder }
    }

    fn construct(&mut self) {
        self.builder.add_cpu("Core i5".to_string());
        self.builder.add_ram(16);
    }

    fn high_spec_construct(&mut self) {
        self.builder.add_cpu("M2".to_string());
        self.builder.add_ram(64);
    }
}

pub struct BuilderMain;

impl BuilderMain {
    pub fn index() {
        let desktop_builder = DesktopBuilder::new();
        let mut desktop_director = Director::new(desktop_builder.clone());
        desktop_director.construct();
        let desktop_computer = desktop_builder.get_result();
        println!("{:?}", desktop_computer);

        let laptop_builder = LaptopBuilder::new();
        let mut laptop_director = Director::new(laptop_builder.clone());
        laptop_director.high_spec_construct();
        let laptop_computer = laptop_builder.get_result();
        println!("{:?}", laptop_computer);
    }
}