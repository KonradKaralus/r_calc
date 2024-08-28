use std::{collections::HashMap, hash::Hash};

use crate::Token;

pub struct OperatorStore {
    operators: HashMap<String, Box<dyn Operator>>,
    matching: String,
    current_matching: Vec<String>,
}

impl OperatorStore {
    pub fn new() -> Self {
        let mut new = Self {
            operators: HashMap::new(),
            matching: "".into(),
            current_matching: vec![],
        };

        new.register_op(Box::new(Plus));
        new.register_op(Box::new(Minus));
        new.register_op(Box::new(Mul));
        new.register_op(Box::new(Div));
        new.register_op(Box::new(Pow));
        new.register_op(Box::new(Sin));
        new.register_op(Box::new(Tan));
        new.register_op(Box::new(Cos));

        new.new_match_step();

        new
    }

    fn register_op(&mut self, operator: Box<dyn Operator>) {
        self.operators.insert(operator.code(), operator);
    }

    pub fn new_match_step(&mut self) {
        self.matching.clear();
        for op in &self.operators {
            self.current_matching.push(op.1.code());
        }
    }

    pub fn match_step(&mut self, next: char) -> (bool, Option<Token>, u32) {
        self.matching.push(next);

        self.current_matching
            .retain(|code| code.starts_with(&self.matching));

        if self.current_matching.len() == 1 {
            let s = self.current_matching.remove(0);
            let skip = s.len() - self.matching.len();
            let ret = Option::from(Token::Op(s));
            self.new_match_step();

            return (true, ret, skip as u32);
        }

        (false, Option::None, 0)
    }

    pub fn get_precedence(&self, code: &String) -> u8 {
        if code == "(" || code == ")" {
            return 255;
        }

        return self
            .operators
            .get(code)
            .unwrap_or_else(|| panic!("unknown op"))
            .precedence();
    }

    pub fn calc_and_no_of_arguments(&self, code: &String) -> (Calculation, u8) {
        let op = self
            .operators
            .get(code)
            .unwrap_or_else(|| panic!("unknown op"));

        (op.calc(), op.no_of_operands())
    }
}

pub type Calculation = Box<dyn Fn(Vec<f64>) -> f64>;

pub trait Operator {
    fn precedence(&self) -> u8;
    fn code(&self) -> String;
    fn calc(&self) -> Calculation;
    fn no_of_operands(&self) -> u8;
}

impl Eq for dyn Operator {}
impl Hash for dyn Operator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.code().hash(state);
        state.finish();
    }
}
impl PartialEq for dyn Operator {
    fn eq(&self, other: &Self) -> bool {
        other.code() == self.code()
    }
}

pub struct Plus;

impl Operator for Plus {
    fn code(&self) -> String {
        "+".into()
    }
    fn precedence(&self) -> u8 {
        1
    }

    fn calc(&self) -> Calculation {
        Box::new(|x| x[0] + x[1])
    }

    fn no_of_operands(&self) -> u8 {
        2
    }
}

pub struct Minus;

impl Operator for Minus {
    fn code(&self) -> String {
        "-".into()
    }
    fn precedence(&self) -> u8 {
        1
    }

    fn calc(&self) -> Calculation {
        Box::new(|x| x[0] - x[1])
    }

    fn no_of_operands(&self) -> u8 {
        2
    }
}

pub struct Mul;

impl Operator for Mul {
    fn code(&self) -> String {
        "*".into()
    }
    fn precedence(&self) -> u8 {
        2
    }

    fn calc(&self) -> Calculation {
        Box::new(|x| x[0] * x[1])
    }

    fn no_of_operands(&self) -> u8 {
        2
    }
}

pub struct Div;

impl Operator for Div {
    fn code(&self) -> String {
        "/".into()
    }
    fn precedence(&self) -> u8 {
        2
    }

    fn calc(&self) -> Calculation {
        Box::new(|x| x[0] / x[1])
    }

    fn no_of_operands(&self) -> u8 {
        2
    }
}

pub struct Pow;

impl Operator for Pow {
    fn code(&self) -> String {
        "p".into()
    }
    fn precedence(&self) -> u8 {
        3
    }

    fn calc(&self) -> Calculation {
        Box::new(|x| x[0].powf(x[1]))
    }

    fn no_of_operands(&self) -> u8 {
        2
    }
}

pub struct Sin;

impl Operator for Sin {
    fn code(&self) -> String {
        "sin".into()
    }
    fn precedence(&self) -> u8 {
        4
    }

    fn calc(&self) -> Calculation {
        Box::new(|x| x[0].sin())
    }

    fn no_of_operands(&self) -> u8 {
        1
    }
}

pub struct Cos;

impl Operator for Cos {
    fn code(&self) -> String {
        "cos".into()
    }
    fn precedence(&self) -> u8 {
        4
    }

    fn calc(&self) -> Calculation {
        Box::new(|x: Vec<f64>| x[0].cos())
    }

    fn no_of_operands(&self) -> u8 {
        1
    }
}

pub struct Tan;

impl Operator for Tan {
    fn code(&self) -> String {
        "tan".into()
    }
    fn precedence(&self) -> u8 {
        4
    }

    fn calc(&self) -> Calculation {
        Box::new(|x: Vec<f64>| x[0].tan())
    }

    fn no_of_operands(&self) -> u8 {
        1
    }
}
