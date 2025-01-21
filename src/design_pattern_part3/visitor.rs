use std::any::{Any, TypeId};

trait Entry: Any {
    fn code(&self) -> &str;
    fn name(&self) -> &str;
    fn get_children(&self) -> &[Box<dyn Entry>];
    fn accept(&self, visior: &mut dyn Visior);
}

struct Group {
    code: String,
    name: String,
    entries: Vec<Box<dyn Entry>>,
}

impl Group {
    fn new(code: String, name: String) -> Self {
        Self {
            code,
            name,
            entries: Vec::new(),
        }
    }

    fn add(&mut self, entry: Box<dyn Entry>) {
        self.entries.push(entry);
    }
}

impl Entry for Group {
    fn code(&self) -> &str {
        &self.code
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn get_children(&self) -> &[Box<dyn Entry>] {
        &self.entries
    }

    fn accept(&self, visior: &mut dyn Visior) {
        visior.visit(self)
    }
}

struct Employee {
    code: String,
    name: String,
}

impl Employee {
    fn new(code: String, name: String) -> Self {
        Self {
            code,
            name,
        }
    }
}

impl Entry for Employee {
    fn code(&self) -> &str {
        &self.code
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn get_children(&self) -> &[Box<dyn Entry>] {
        &[]
    }

    fn accept(&self, visior: &mut dyn Visior) {
        visior.visit(self)
    }
}

trait Visior {
    fn visit(&mut self, entry: &dyn Entry);
}

struct ListVisitor;

impl ListVisitor {
    fn new() -> Self {
        Self
    }
}

impl Visior for ListVisitor {
    fn visit(&mut self, entry: &dyn Entry) {
        if entry.type_id() == TypeId::of::<Group>() {
            println!("{}: {}", entry.code(), entry.name());
        } else {
            println!("    {}: {}", entry.code(), entry.name());
        }

        for child in entry.get_children() {
            child.accept(self);
        }
    }
}

struct CountVisitor {
    group_count: i64,
    employee_count: i64,
}

impl CountVisitor {
    fn new() -> Self {
        Self {
            group_count: 0,
            employee_count: 0,
        }
    }
}

impl Visior for CountVisitor {
    fn visit(&mut self, entry: &dyn Entry) {
        if entry.type_id() == TypeId::of::<Group>() {
            self.group_count += 1;
        } else {
            self.employee_count += 1;
        }

        for child in entry.get_children() {
            child.accept(self);
        }
    }
}

pub struct VisitorMain;

impl VisitorMain {
    pub fn index() {
        let mut root_entry = Group::new("01".to_string(), "02".to_string());
        root_entry.add(Box::new(Employee::new("0101".to_string(), "社長".to_string())));
        root_entry.add(Box::new(Employee::new("0102".to_string(), "副社長".to_string())));

        let mut group1 = Group::new("11".to_string(), "神奈川支部".to_string());
        group1.add(Box::new(Employee::new("1001".to_string(), "支部長".to_string())));

        let mut group2 = Group::new("11".to_string(), "横浜営業所".to_string());
        group2.add(Box::new(Employee::new("1101".to_string(), "営業部長".to_string())));
        group2.add(Box::new(Employee::new("1102".to_string(), "yamada".to_string())));
        group2.add(Box::new(Employee::new("1103".to_string(), "suzuki".to_string())));
        group2.add(Box::new(Employee::new("1104".to_string(), "tanaka".to_string())));

        group1.add(Box::new(group2));
        root_entry.add(Box::new(group1));

        let mut list_visitor = ListVisitor::new();
        let mut count_visitor = CountVisitor::new();

        root_entry.accept(&mut list_visitor);
        root_entry.accept(&mut count_visitor);

        println!("グループ数:{}", count_visitor.group_count);
        println!("社員数:{}", count_visitor.employee_count);
        
    }
}