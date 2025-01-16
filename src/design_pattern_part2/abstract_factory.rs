trait Button {
    fn press(&self);
}

trait CheckBox {
    fn switch(&self);
}

trait GUIFactory {
    fn create_button(&self) -> impl Button;
    fn create_checkbox(&self) -> impl CheckBox;
}

struct WindowsButton;

impl WindowsButton {
    fn new() -> Self { Self }
}

impl Button for WindowsButton {
    fn press(&self) {
        println!("Windowsのボタンが押されました");
    }
}

struct WindowsCheckBox;

impl WindowsCheckBox {
    fn new() -> Self { Self }
}

impl CheckBox for WindowsCheckBox {
    fn switch(&self) {
        println!("Windowsのチェックボックスが切り替えられました");
    }
}

struct WindowsGUIFactory;

impl WindowsGUIFactory {
    fn new() -> Self { Self }
}

impl GUIFactory for WindowsGUIFactory {
    fn create_button(&self) -> impl Button {
        WindowsButton::new()
    }

    fn create_checkbox(&self) -> impl CheckBox {
        WindowsCheckBox::new()
    }
}

struct MacButton;

impl MacButton {
    fn new() -> Self { Self }
}

impl Button for MacButton {
    fn press(&self) {
        println!("Macのボタンが押されました");
    }
}

struct MacCheckBox;

impl MacCheckBox {
    fn new() -> Self { Self }
}

impl CheckBox for MacCheckBox {
    fn switch(&self) {
        println!("Macのチェックボックスが切り替えられました");
    }
}

struct MacGUIFactory;

impl MacGUIFactory {
    fn new() -> Self { Self }
}

impl GUIFactory for MacGUIFactory {
    fn create_button(&self) -> impl Button {
        MacButton::new()
    }

    fn create_checkbox(&self) -> impl CheckBox {
        MacCheckBox::new()
    }
}

pub struct AbstractFactoryMain;

impl AbstractFactoryMain {
    pub fn index() {
        Self::run(WindowsGUIFactory::new());
        Self::run(MacGUIFactory::new());
    }

    fn run(factory: impl GUIFactory) {
        let button = factory.create_button();
        let checkbox = factory.create_checkbox();
        button.press();
        checkbox.switch();
    }
}