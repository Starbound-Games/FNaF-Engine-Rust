use std::f64::consts::PI;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Operator(char),
    Function(String),
    ParenL,
    ParenR,
}

struct MathEvaluator;

impl MathEvaluator {
    pub fn evaluate(expression: &str) -> Result<f64, String> {
        let tokens = Self::tokenize(expression)?;
        let (ast, _) = Self::parse(&tokens)?;
        Ok(Self::evaluate_ast(&ast))
    }

    fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = expression.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                ' ' => continue,
                '+' | '*' | '/' => tokens.push(Token::Operator(c)),
                '-' => {
                    // Check if '-' is indicating a negative number
                    if tokens.is_empty() || matches!(tokens.last().unwrap(), Token::Operator(_) | Token::ParenL) {
                        let mut num = String::from("-");
                        while chars.peek().map_or(false, |&ch| ch.is_digit(10) || ch == '.') {
                            num.push(chars.next().unwrap());
                        }
                        tokens.push(Token::Number(num.parse().map_err(|_| "Invalid number")?));
                    } else {
                        tokens.push(Token::Operator(c));
                    }
                }
                '(' => tokens.push(Token::ParenL),
                ')' => tokens.push(Token::ParenR),
                '0'..='9' => {
                    let mut num = c.to_string();
                    while chars.peek().map_or(false, |&ch| ch.is_digit(10) || ch == '.') {
                        num.push(chars.next().unwrap());
                    }
                    tokens.push(Token::Number(num.parse().map_err(|_| "Invalid number")?));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut func = c.to_string();
                    while chars.peek().map_or(false, |&ch| ch.is_alphabetic()) {
                        func.push(chars.next().unwrap());
                    }
                    if func == "sin" || func == "cos" {
                        tokens.push(Token::Function(func));
                    } else {
                        return Err("Unknown function".into());
                    }
                }
                _ => return Err("Unexpected character".into()),
            }
        }
        Ok(tokens)
    }

    fn parse(tokens: &[Token]) -> Result<(Vec<Token>, usize), String> {
        let mut output = Vec::new();
        let mut operators = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                Token::Number(_) | Token::Function(_) => output.push(tokens[i].clone()),
                Token::Operator(op) => {
                    while let Some(top) = operators.last() {
                        if Self::precedence(top) >= Self::precedence(&tokens[i]) {
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operators.push(tokens[i].clone());
                }
                Token::ParenL => operators.push(tokens[i].clone()),
                Token::ParenR => {
                    while let Some(top) = operators.pop() {
                        if top == Token::ParenL {
                            break;
                        }
                        output.push(top);
                    }
                }
            }
            i += 1;
        }

        while let Some(op) = operators.pop() {
            output.push(op);
        }

        Ok((output, i))
    }

    fn precedence(token: &Token) -> usize {
        match token {
            Token::Operator(c) => match c {
                '+' | '-' => 1,
                '*' | '/' => 2,
                _ => 0,
            },
            _ => 0,
        }
    }

    fn evaluate_ast(tokens: &[Token]) -> f64 {
        let mut stack = Vec::new();

        for token in tokens {
            match token {
                Token::Number(n) => stack.push(*n),
                Token::Operator(op) => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let result = match op {
                        '+' => a + b,
                        '-' => a - b,
                        '*' => a * b,
                        '/' => a / b,
                        _ => panic!("Unexpected operator"),
                    };
                    stack.push(result);
                }
                Token::Function(func) => {
                    let arg = stack.pop().unwrap();
                    let result = match func.as_str() {
                        "sin" => (arg * PI / 180.0).sin(),
                        "cos" => (arg * PI / 180.0).cos(),
                        _ => panic!("Unknown function"),
                    };
                    stack.push(result);
                }
                _ => panic!("Unexpected token"),
            }
        }

        stack.pop().unwrap()
    }
}