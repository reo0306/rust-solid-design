trait TestTemplate {
    fn test(&self) {
        self.setup();
        self.execute();
        self.teardown();
    }
    fn setup(&self);
    fn execute(&self);
    fn teardown(&self) {
        println!("teardown");
    }
}

struct ItemServiceTest;

impl ItemServiceTest {
    fn new() -> Self {
        Self { }
    }
}

impl TestTemplate for ItemServiceTest {
    fn setup(&self) {
        println!("setup: ItemServiceTest");
    }

    fn execute(&self) {
        println!("execute: ItemServiceTest");
    }
}

struct UserServiceTest;

impl UserServiceTest {
    fn new() -> Self {
        Self { }
    }
}

impl TestTemplate for UserServiceTest {
    fn setup(&self) {
        println!("setup: UserServiceTest");
    }

    fn execute(&self) {
        println!("execute: UserServiceTest");
    }
}

pub struct TemplateMain;

impl TemplateMain {
    pub fn index() {
        let item_service_test = ItemServiceTest::new();
        let user_service_test = UserServiceTest::new();

        item_service_test.test();
        user_service_test.test();
    }
}