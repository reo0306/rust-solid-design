use regex::Regex;
use chrono::{DateTime, Utc, Datelike};

struct Context {
    expression: String,
    date: DateTime<Utc>
}

impl Context {
    fn new(expression: String, date: DateTime<Utc>) -> Self {
        Self {
            expression,
            date,
        }
    }

    fn validate(&self) {
        let re = Regex::new(".*MM.*DD.*YYYY.*").unwrap();
        let result = re.is_match(&self.expression);
        if self.expression.len() != 10 || !result {
            panic!("expressionが不正です")
        }
    }
}

trait AbstractExpression {
    fn interpret(&self, context: Context) -> Context;
}

struct YearExpression {
    child: Option<Box<dyn AbstractExpression>>,
}

impl YearExpression {
    fn new() -> Self { Self { child: None }}

    fn set_child(&mut self, child: impl AbstractExpression + 'static) {
        self.child = Some(Box::new(child));
    }
}

impl AbstractExpression for YearExpression {
    fn interpret(&self, mut context: Context) -> Context {
        let year = context.date.year();
        context.expression = context.expression.replace("YYYY", &year.to_string());

        if let Some(ref child) = self.child {
            return child.interpret(context);
        }

        context
    }
}

struct MonthExpression {
    child: Option<Box<dyn AbstractExpression>>,
}

impl MonthExpression {
    fn new() -> Self {
        Self {
            child: None,
        }
    }

    fn set_child(&mut self, child: impl AbstractExpression + 'static) {
        self.child = Some(Box::new(child));
    }
}

impl AbstractExpression for MonthExpression {
    fn interpret(&self, mut context: Context) -> Context {
        let month = context.date.month();
        context.expression = context.expression.replace("MM", &month.to_string());

        if let Some(ref child) = self.child {
            return child.interpret(context);
        }

        context
    }
}

struct DayExpression {
    child: Option<Box<dyn AbstractExpression>>,
}

impl DayExpression {
    fn new() -> Self { Self { child: None }}

    fn set_child(&mut self, child: impl AbstractExpression + 'static) {
        self.child = Some(Box::new(child));
    }
}

impl AbstractExpression for DayExpression {
    fn interpret(&self, mut context: Context) -> Context {
        let year = context.date.day();
        context.expression = context.expression.replace("DD", &year.to_string());

        if let Some(ref child) = self.child {
            return child.interpret(context);
        }

        context
    }
}

pub struct InterpreterMain;

impl InterpreterMain {
    pub fn index() {
        let now_date = Utc::now();
        let expression = "MM/DD/YYYY".to_string();
        let context = Context::new(expression, now_date);
        context.validate();

        let mut year_expression = YearExpression::new();
        let mut month_expression = MonthExpression::new();
        let mut day_expression = DayExpression::new();
        month_expression.set_child(day_expression);
        year_expression.set_child(month_expression);

        let result = year_expression.interpret(context);

        println!("{:?}", now_date);
        println!("{:?}", result.expression);
    }
}