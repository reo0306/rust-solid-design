use regex::Regex;
use std::sync::Arc;

trait ValidationHandler: Send + Sync  {
    fn set_handler(&mut self, handler: Arc<dyn ValidationHandler>);
    fn exec_validation(&self, input: &str) -> bool;
    fn get_error_message(&self);
    fn validate(&self, input: &str) -> bool {
        let result = self.exec_validation(input);

        if !result {
            self.get_error_message();
            return false;
        }
        
        if let Some(next_handler) = self.get_next_handler() {
            return next_handler.validate(input);
        }

        true
    }

    fn get_next_handler(&self) -> Option<&Arc<dyn ValidationHandler>>;
}

#[derive(Default)]
struct NotNullValidationHandler {
    next_handler: Option<Arc<dyn ValidationHandler>>,
}

impl NotNullValidationHandler {
    fn new() -> Self {
        Default::default()
    }
}

impl ValidationHandler for NotNullValidationHandler {
    fn set_handler(&mut self, handler: Arc<dyn ValidationHandler>) {
        self.next_handler = Some(handler);
    }

    fn exec_validation(&self, input: &str) -> bool {
        let result = !input.is_empty();
        println!("NotNullValidationの結果: {}", result);

        result
    }

    fn get_error_message(&self) {
        println!("何も入力されていません");
    }

    fn get_next_handler(&self) -> Option<&Arc<dyn ValidationHandler>> {
        self.next_handler.as_ref()
    }
}

#[derive(Default)]
struct AlphabetValidationHandler {
    next_handler: Option<Arc<dyn ValidationHandler>>
}

impl AlphabetValidationHandler{
    fn new() -> Self {
        Default::default()
    }
}

impl ValidationHandler for AlphabetValidationHandler {
    fn set_handler(&mut self, handler: Arc<dyn ValidationHandler>) {
        self.next_handler = Some(handler);
    }

    fn exec_validation(&self, input: &str) -> bool {
        let re = Regex::new("^[a-zA-z]+$").unwrap();
        let result = re.is_match(&input);
    
        println!("AplabetVallidationHandelrの結果: {}", result);

        result
    }

    fn get_error_message(&self) {
        println!("半角英字で入力してください");
    }

    fn get_next_handler(&self) -> Option<&Arc<dyn ValidationHandler>> {
        self.next_handler.as_ref()
    }
}

#[derive(Default)]
struct MinLengthValidationHandeler {
    next_handler: Option<Arc<dyn ValidationHandler>>,
}

impl MinLengthValidationHandeler {
    fn new() -> Self {
        Default::default()
    }
}

impl ValidationHandler for MinLengthValidationHandeler {
    fn set_handler(&mut self, handler: Arc<dyn ValidationHandler>) {
        self.next_handler = Some(handler);
    }

    fn exec_validation(&self, input: &str) -> bool {
        let result = input.len() >= 8;
        println!("MinLengthVallidationHandelrの結果: {}", result);

        result
    }

    fn get_error_message(&self) {
        println!("8文字以上で入力してください");
    }

    fn get_next_handler(&self) -> Option<&Arc<dyn ValidationHandler>> {
        self.next_handler.as_ref()
    }
}

pub struct ChainOfResponsibiltyMain;

impl ChainOfResponsibiltyMain {
    pub fn index() {
        let mut not_null_handler = NotNullValidationHandler::new();
        let mut alphabe_handelr = AlphabetValidationHandler::new();
        let min_length_handler = MinLengthValidationHandeler::new();

        alphabe_handelr.set_handler(Arc::new(min_length_handler));
        not_null_handler.set_handler(Arc::new(alphabe_handelr));

        let result = not_null_handler.validate("helloworld");

        if result {
            println!("全てのバリデーションに通過しました");
        }
    }
}