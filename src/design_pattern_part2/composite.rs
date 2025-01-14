trait Entry {
    fn get_name(&self) -> String;
    fn get_size(&self) -> i64;
    fn remove(&self);
}

struct File {
    name: String,
    size: i64,
}

impl File {
    fn new(name: String, size: i64) -> Self {
        Self {
            name,
            size,
        }
    }
}

impl Entry for File {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> i64 {
        self.size
    }

    fn remove(&self) {
        println!("{}を削除しました", self.name);
    }
}

struct Directory<T: Entry> {
    name: String,
    children: Vec<T>,
}

impl<T: Entry> Directory<T> {
    fn new(name: String) -> Self {
        Self {
            name,
            children: Vec::new(),
        }
    }

    fn add(&mut self, child: T) {
        self.children.push(child)
    }
}

impl<T: Entry> Entry for Directory<T> {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> i64 {
        let mut size = 0;

        for child in &self.children {
            size += child.get_size();
        }

        size
    }

    fn remove(&self) {
        for child in &self.children {
            child.remove();
        }

        println!("{}を削除しました", self.name);
    }
}

pub struct CompositeMain;

impl CompositeMain {
    pub fn index() {
        let mut dir1 = Directory::new("design_pattern".to_string());
        let mut dir2 = Directory::new("composite".to_string());
        let file1 = File::new("composite.py".to_string(), 100);
        let file2 = File::new("practice.png".to_string(), 150);

        dir2.add(file1);
        dir2.add(file2);
        dir1.add(dir2);

        Self::client(&dir1);
    }

    fn client(entry: &dyn Entry) {
        println!("{}", entry.get_name());
        println!("{}", entry.get_size());
        entry.remove();
    }
}