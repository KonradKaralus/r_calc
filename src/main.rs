use clap::Parser;
use std::collections::VecDeque;

use cli::Cli;
use operator::OperatorStore;

mod cli;
mod operator;
mod tests;

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Op(String),
    Bracket(char),
}

fn l_bracket() -> Token {
    Token::Bracket('(')
}
fn r_bracket() -> Token {
    Token::Bracket(')')
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}
fn main() {
    let cli = Cli::parse();

    let expr = cli.expression;
    let prec = cli.precise;

    println!("{}", process(expr, prec));
}

fn process(input: String, precise: bool) -> f64 {
    let mut os = OperatorStore::new();

    let mut tokens = tokenizer(&input, &mut os);

    tokens = process_negatives(tokens);

    let mut res = calc(eval(tokens, &os), &os);

    if !precise {
        res = round(res, 2);
    }

    res
}

fn process_negatives(mut input: Vec<Token>) -> Vec<Token> {
    if !input.contains(&Token::Op("-".into())) {
        return input;
    }

    if *input.first().unwrap() == Token::Op("-".into()) {
        input.insert(2, r_bracket());
        input.insert(0, Token::Number(0f64));
        input.insert(0, l_bracket());
    }

    let mut ins_pos: Vec<usize> = vec![];

    for (pos, t) in input.iter().enumerate() {
        if *t == Token::Bracket('(') && *input.get(pos + 1).unwrap() == Token::Op("-".into()) {
            ins_pos.push(pos + 1);
        }
    }

    ins_pos.reverse();

    for pos in ins_pos {
        input.insert(pos, Token::Number(0f64));
    }

    input
}

fn tokenizer(input: &str, operators: &mut OperatorStore) -> Vec<Token> {
    let mut current = "".to_string();
    let mut tokens = vec![];
    let mut is_number = input.chars().next().unwrap().is_numeric();
    let mut new = true;
    let mut skip = 0;

    for c in input.chars() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if is_number && !(c.is_numeric() || c == '.') {
            let number: f64 = current.parse().unwrap();
            tokens.push(Token::Number(number));
            new = true;
        }

        if new {
            new = false;
            current.clear();
            is_number = c.is_numeric();
            if ['(', ')'].contains(&c) {
                tokens.push(Token::Bracket(c));
                new = true;
                continue;
            }
        }

        if !is_number {
            let m = operators.match_step(c);

            if m.0 {
                tokens.push(m.1.unwrap());
                skip = m.2;
                new = true;
                continue;
            }

            current.push(c);
        } else {
            current.push(c);
        }
    }

    if is_number {
        let number: f64 = current.parse().unwrap();
        tokens.push(Token::Number(number));
    }

    tokens
}

// Shunting-Yard-Algorithm
fn eval(input: Vec<Token>, operators: &OperatorStore) -> VecDeque<Token> {
    use {Token::Bracket, Token::Number, Token::Op};

    let mut output_queue: VecDeque<Token> = VecDeque::new();
    let mut operator_stack: Vec<String> = vec![];

    for t in input {
        match t {
            Number(_) => output_queue.push_back(t),

            Op(o) => {
                if !operator_stack.is_empty() {
                    loop {
                        let last = operator_stack.last().unwrap();
                        let last_precedence = operators.get_precedence(last);
                        let precedence = operators.get_precedence(&o);

                        if last_precedence >= precedence && last_precedence != 255 {
                            output_queue.push_back(Op(operator_stack.pop().unwrap()));
                        } else {
                            break;
                        }
                        if operator_stack.is_empty() {
                            break;
                        }
                    }
                }
                operator_stack.push(o);
            }

            Bracket(b) => {
                if b == '(' {
                    operator_stack.push("(".into());
                } else if b == ')' && !operator_stack.is_empty() {
                    loop {
                        let last = operator_stack.last().unwrap();
                        if last != "(" {
                            output_queue.push_back(Token::Op(last.clone()));
                            operator_stack.pop();
                        } else {
                            operator_stack.pop();
                            break;
                        }
                        if operator_stack.is_empty() {
                            break;
                        }
                    }
                }
            }
        }
    }

    loop {
        let op = operator_stack.pop();
        match op {
            Some(o) => {
                if o != "(" && o != ")" {
                    output_queue.push_back(Op(o))
                }
            }
            None => break,
        }
    }

    output_queue
}

fn calc(mut input: VecDeque<Token>, operators: &OperatorStore) -> f64 {
    use {Token::Number, Token::Op};

    let mut stack: Vec<f64> = vec![];
    while !input.is_empty() {
        let token = input.pop_front().unwrap();
        match token {
            Number(i) => stack.push(i),
            Op(o) => {
                let calc = operators.calc_and_no_of_arguments(&o);
                let mut operands = vec![];
                for _ in 0..calc.1 {
                    operands.push(stack.pop().unwrap());
                }
                operands.reverse();
                let res = calc.0(operands);

                stack.push(res);
            }
            _ => panic!("{:?} got into ended up in output stack", token),
        }
    }

    stack[0]
}
