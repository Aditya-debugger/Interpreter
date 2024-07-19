use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
    EOF,
}

struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.input.chars().next();
        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = if self.position < self.input.len() {
            Some(self.input.chars().nth(self.position).unwrap())
        } else {
            None
        };
    }

    fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.current_char {
            match c {
                '0'..='9' | '.' => tokens.push(self.number()),
                ' ' | '\t' | '\n' | '\r' => self.advance(),
                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }
                _ if c.is_alphabetic() => tokens.push(self.identifier()),
                _ => panic!("Unexpected character: {}", c),
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn number(&mut self) -> Token {
        let start_pos = self.position;
        while let Some(c) = self.current_char {
            if c.is_numeric() || c == '.' {
                self.advance();
            } else {
                break;
            }
        }
        let number_str: String = self.input[start_pos..self.position].to_string();
        Token::Number(number_str.parse::<f64>().unwrap())
    }

    fn identifier(&mut self) -> Token {
        let start_pos = self.position;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        let ident: String = self.input[start_pos..self.position].to_string();
        match ident.as_str() {
            "plus" => Token::Plus,
            "minus" => Token::Minus,
            "mul" => Token::Mul,
            "div" => Token::Div,
            _ => panic!("Unexpected identifier: {}", ident),
        }
    }
}

#[derive(Debug)]
enum ASTNode {
    Number(f64),
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn parse(&mut self) -> ASTNode {
        self.expression()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn expression(&mut self) -> ASTNode {
        self.term()
    }

    fn term(&mut self) -> ASTNode {
        let mut node = self.factor();
        while let Token::Plus | Token::Minus = self.current_token() {
            let op = self.current_token().clone();
            self.advance();
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(self.factor()));
        }
        node
    }

    fn factor(&mut self) -> ASTNode {
        let mut node = self.primary();
        while let Token::Mul | Token::Div = self.current_token() {
            let op = self.current_token().clone();
            self.advance();
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(self.primary()));
        }
        node
    }

    fn primary(&mut self) -> ASTNode {
        match self.current_token() {
            Token::Number(n) => {
                let value = *n;
                self.advance();
                ASTNode::Number(value)
            }
            Token::LParen => {
                self.advance();
                let node = self.expression();
                self.expect(Token::RParen);
                node
            }
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }

    fn expect(&mut self, expected: Token) {
        if *self.current_token() == expected {
            self.advance();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current_token());
        }
    }
}

struct Interpreter;

impl Interpreter {
    fn new() -> Self {
        Interpreter
    }

    fn interpret(&mut self, node: &ASTNode) -> f64 {
        match node {
            ASTNode::Number(n) => *n,
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.interpret(left);
                let right_val = self.interpret(right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Mul => left_val * right_val,
                    Token::Div => left_val / right_val,
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("Enter expression: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(input);
        let tokens = lexer.get_tokens();
        println!("{:?}", tokens);

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        println!("{:?}", ast);

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&ast);
        println!("Result: {}", result);
    }
}
