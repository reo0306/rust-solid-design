use std::collections::HashMap;
use std::any::Any;

trait ItemPrototype {
    fn create_copy(&self) -> Box<dyn ItemPrototype>;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug)]
struct DeepCopyItem {
    name: String,
    review: Vec<String>,
}

impl DeepCopyItem {
    fn new(name: String) -> Self {
        Self {
            name: name,
            review: Vec::new(),
        }
    }

    fn set_review(&mut self, review: String) {
        self.review.push(review);
    }
}

impl ItemPrototype for DeepCopyItem {
    fn create_copy(&self) -> Box<dyn ItemPrototype> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
struct ShallowCopyItem {
    name: String,
    review: Vec<String>,
}

impl ShallowCopyItem {
    fn new(name: String) -> Self {
        Self {
            name: name,
            review: Vec::new(),
        }
    }

    fn set_review(&mut self, review: String) {
        self.review.push(review);
    }
}

impl ItemPrototype for ShallowCopyItem {
    fn create_copy(&self) -> Box<dyn ItemPrototype> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct ItemManager {
    items: HashMap<String, Box<dyn ItemPrototype>>,
}

impl ItemManager {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn register_item(&mut self, key: String, item: Box<dyn ItemPrototype>) {
        self.items.insert(key, item);
    }

    fn create(&self, key: &str) -> Box<dyn ItemPrototype> {
        if let Some(item) = self.items.get(key) {
            item.create_copy()
        } else {
            panic!("指定されたキーが存在しません")
        }
    }
}

pub struct ProtoTypeMain;

impl ProtoTypeMain {
    pub fn index() {
        let mouse = Box::new(DeepCopyItem::new("マウス".to_string())) as Box<dyn ItemPrototype>;
        //println!("mouse(original) {:?}", mouse);
        let keyboard = Box::new(ShallowCopyItem::new("キーボード".to_string())) as Box<dyn ItemPrototype>;
        //println!("keyboard(original) {:?}", keyboard);

        let mut manager = ItemManager::new();
        manager.register_item("mouse".to_string(), mouse);
        manager.register_item("keyboard".to_string(), keyboard);

        let mut cloned_mouse = manager.create("mouse");
        let mut cloned_keyboard = manager.create("keyboard");

        if let Some(deep_mouse) = cloned_mouse.as_any().downcast_ref::<DeepCopyItem>() {
            //deep_mouse.set_review("Good".to_string());
            println!("mouse(copy) {:?}", deep_mouse);
        }


        if let Some(shallow_keyboard) = cloned_keyboard.as_any().downcast_ref::<ShallowCopyItem>() {
            //shallow_keyboard.set_review("SoSo!".to_string());
            println!("keyboard(copy) {:?}", shallow_keyboard);
        }
    }
}