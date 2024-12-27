pub trait Shape {
    fn get_area(&self) -> i64;
}

pub struct Rectangle {
    width: i64,
    height: i64,
}

impl Shape for Rectangle {
    fn get_area(&self) -> i64 {
        self.width * self.height
    }
}

impl Rectangle {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    fn set_width(&mut self, width: i64) {
        self.width = width;
    }

    fn set_heigh(&mut self, height: i64) {
        self.height = height;
    }
}

pub struct Square {
    length: i64,
}

impl Shape for Square {
    fn get_area(&self) -> i64 {
        self.length.pow(2)
    }
}

impl Square {
    fn new() -> Self {
        Self { length: 0 }
    }

    fn set_length(&mut self, length: i64) {
        self.length = length;
    }
}

pub struct F;

impl F {
    pub fn f(shape: impl Shape) {
        println!("{}", shape.get_area());
    }
}

pub struct LiskovMain;

impl LiskovMain {
    pub fn index() {
        let mut r1 = Rectangle::new();
        r1.set_width(3);
        r1.set_heigh(4);
        F::f(r1);

        let mut r2 = Square::new() ;
        r2.set_length(3);
        F::f(r2);
    }
}