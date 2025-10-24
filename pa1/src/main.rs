use std::env;
use std::fs;
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(String),
    Plus,
    Star,
    Bopen,
    Bclose,
    Eof,
}

impl Token {
    fn to_parse_tree_string(&self) -> String {
        match self {
            Token::Identifier(ref s) => format!("IDENTIFIER({})", s),
            Token::Number(ref s) => format!("NUMBER({})", s),
            Token::Plus => "PLUS".to_string(),
            Token::Star => "STAR".to_string(),
            Token::Bopen => "BOPEN".to_string(),
            Token::Bclose => "BCLOSE".to_string(),
            Token::Eof => "EOF".to_string(),
        }
    }
}

struct Scanner {
    chars: Vec<char>,
    current: usize,
}

impl Scanner {
    fn new(input: &str) -> Self {
        Scanner { chars: input.chars().collect(), current: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.current < self.chars.len() {
            let c = self.chars[self.current];
            self.current += 1;
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |c| c.is_ascii_whitespace()) {
            self.advance();
        }
    }

    fn scan_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let c = match self.advance() {
            Some(c) => c,
            None => return Some(Token::Eof),
        };
        match c {
            '+' => Some(Token::Plus),
            '*' => Some(Token::Star),
            '(' => Some(Token::Bopen),
            ')' => Some(Token::Bclose),
            d if d.is_ascii_digit() => {
                let mut lexeme = d.to_string();
                while self.peek().map_or(false, |n| n.is_ascii_digit()) {
                    lexeme.push(self.advance().unwrap());
                }
                Some(Token::Number(lexeme))
            }
            a if a.is_ascii_alphabetic() => {
                let mut lexeme = a.to_string();
                while self.peek().map_or(false, |n| n.is_ascii_alphabetic()) {
                    lexeme.push(self.advance().unwrap());
                }
                Some(Token::Identifier(lexeme))
            }
            _ => {
                eprintln!("Error: Unexpected character '{}'.", c);
                None
            }
        }
    }

    pub fn scan_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            match self.scan_token() {
                Some(token) => {
                    let is_eof = matches!(token, Token::Eof);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                }
                None => break, // fix: break instead of spinning forever
            }
        }
        tokens
    }
}

pub enum ParseNode {
    NonTerminal(String, Vec<ParseNode>),
    Terminal(Token),
    Epsilon,
}

impl ParseNode {
    pub fn to_bfs_string(&self) -> String {
        let mut current_level_queue: VecDeque<&ParseNode> = VecDeque::new();
        current_level_queue.push_back(self);

        let mut result = String::new();

        while !current_level_queue.is_empty() {
            let mut next_level_queue: VecDeque<&ParseNode> = VecDeque::new();
            let mut level_output: Vec<String> = Vec::new();

            while let Some(node) = current_level_queue.pop_front() {
                match node {
                    ParseNode::NonTerminal(name, children) => {
                        level_output.push(name.clone());
                        for child in children {
                            next_level_queue.push_back(child);
                        }
                    }
                    ParseNode::Terminal(token) => {
                        level_output.push(token.to_parse_tree_string());
                    }
                    ParseNode::Epsilon => {
                        level_output.push("EPSILON".to_string());
                    }
                }
            }

            if !level_output.is_empty() {
                result.push_str(&level_output.join(" "));
                result.push('\n');
            }

            current_level_queue = next_level_queue;
        }
        result
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self) -> Token {
        self.current += 1;
        self.tokens[self.current - 1].clone()
    }

    pub fn parse(&mut self) -> Result<ParseNode, String> {
        let root = self.parse_expr()?;
        if matches!(self.peek(), Token::Eof) {
            self.consume();
            Ok(root)
        } else {
            Err(format!("Parse Error: Extra token found starting at {:?}", self.peek()))
        }
    }

    fn parse_expr(&mut self) -> Result<ParseNode, String> {
        Ok(ParseNode::NonTerminal(
            "EXPR".to_string(),
            vec![self.parse_term()?, self.parse_exprdash()?],
        ))
    }

    fn parse_exprdash(&mut self) -> Result<ParseNode, String> {
        match self.peek() {
            Token::Plus => Ok(ParseNode::NonTerminal(
                "EXPRDASH".to_string(),
                vec![ParseNode::Terminal(self.consume()), self.parse_term()?, self.parse_exprdash()?],
            )),
            Token::Bclose | Token::Eof => Ok(ParseNode::NonTerminal("EXPRDASH".to_string(), vec![ParseNode::Epsilon])),
            _ => Err(format!("Parse Error in EXPRDASH: Expected '+', ')', or EOF, found {:?}", self.peek())),
        }
    }

    fn parse_term(&mut self) -> Result<ParseNode, String> {
        Ok(ParseNode::NonTerminal(
            "TERM".to_string(),
            vec![self.parse_factor()?, self.parse_termdash()?],
        ))
    }

    fn parse_termdash(&mut self) -> Result<ParseNode, String> {
        match self.peek() {
            Token::Star => Ok(ParseNode::NonTerminal(
                "TERMDASH".to_string(),
                vec![ParseNode::Terminal(self.consume()), self.parse_factor()?, self.parse_termdash()?],
            )),
            Token::Plus | Token::Bclose | Token::Eof => Ok(ParseNode::NonTerminal("TERMDASH".to_string(), vec![ParseNode::Epsilon])),
            _ => Err(format!("Parse Error in TERMDASH: Expected '*', '+', ')', or EOF, found {:?}", self.peek())),
        }
    }

    fn parse_factor(&mut self) -> Result<ParseNode, String> {
        let factor_node = match self.peek() {
            Token::Bopen => {
                let mut children = vec![ParseNode::Terminal(self.consume()), self.parse_expr()?];
                if !matches!(self.peek(), Token::Bclose) {
                    return Err(format!("Parse Error in FACTOR: Expected ')', found {:?}", self.peek()));
                }
                children.push(ParseNode::Terminal(self.consume()));
                ParseNode::NonTerminal("FACTOR".to_string(), children)
            }
            Token::Identifier(_) | Token::Number(_) => {
                ParseNode::NonTerminal("FACTOR".to_string(), vec![ParseNode::Terminal(self.consume())])
            }
            _ => {
                return Err(format!(
                    "Parse Error in FACTOR: Expected '(', Identifier, or Number, found {:?}",
                    self.peek()
                ))
            }
        };
        Ok(factor_node)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./scanparse <input_filename>");
        return;
    }

    let input_path = PathBuf::from(&args[1]);

    let absolute_input_path = match input_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "Error canonicalizing path {}: {}. Ensure file exists.",
                input_path.display(),
                e
            );
            return;
        }
    };
    eprintln!("Attempting to read from: {}", absolute_input_path.display());

    let content = match fs::read_to_string(&absolute_input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Error reading file {}: {}",
                absolute_input_path.display(),
                e
            );
            return;
        }
    };

    let mut output_path = absolute_input_path.clone();

    if let Some(stem) = output_path.file_stem().and_then(|s| s.to_str()) {
        let stem_str = stem.to_string();     
        output_path.set_file_name(stem_str); 
    } else {
        eprintln!("Error: Input path is not a valid file path.");
        return;
    }

    output_path.set_extension("output");


    let mut all_output = String::new();

    for (line_num, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut scanner = Scanner::new(line);
        let tokens = scanner.scan_all();

        if tokens.is_empty() || !matches!(tokens.last(), Some(Token::Eof)) {
            if !line.trim().is_empty() {
                eprintln!(
                    "Scanning Error: Could not tokenize line {}: '{}'",
                    line_num + 1,
                    line
                );
            }
            continue;
        }

        match Parser::new(tokens).parse() {
            Ok(root) => {
                let bfs_string = root.to_bfs_string();
                print!("{}", bfs_string);
                all_output.push_str(&bfs_string);
            }
            Err(e) => {
                eprintln!("Parse Error on line {}: '{}'", line_num + 1, line);
                eprintln!("{}", e);
            }
        }
    }

    match fs::write(&output_path, all_output) {
        Ok(_) => {
            eprintln!(
                "Successfully wrote parse trees to file: {}",
                output_path.display()
            );
        }
        Err(e) => {
            eprintln!(
                "ERROR: Failed to write output to file {}: {}",
                output_path.display(),
                e
            );
        }
    }
}
