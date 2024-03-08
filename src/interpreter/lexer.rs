use crate::language::command::{Command, CommandAction, Params};
use crate::language::dictionary::CommandDictionary;
use crate::language::token::Token;
use std::collections::VecDeque;
use std::error::Error;

pub struct Lexer {
    dictionary: CommandDictionary,
    stack: VecDeque<LexerFrame>,
}

impl Lexer {
    pub fn with(dictionary: CommandDictionary) -> Self {
        Lexer {
            dictionary,
            stack: VecDeque::new(),
        }
    }

    pub fn push_frame(&mut self, text: &str, in_paren: bool) {
        let frame = LexerFrame {
            text: String::from(text),
            position: 0,
            in_paren,
        };
        self.stack.push_back(frame);
    }

    pub fn pop_frame(&mut self) -> bool {
        let exiting_main = self.stack.len() == 1;
        if self.stack.len() > 0 {
            self.stack.pop_back();
        }
        exiting_main
    }

    pub fn clear_frames(&mut self) {
        self.stack.clear();
    }

    pub fn return_to_start_of_top_frame(&mut self) {
        let top_frame = self.stack.back_mut().unwrap();
        top_frame.position = 0;
    }

    fn get_top_frame(&mut self) -> &mut LexerFrame {
        self.stack.back_mut().unwrap()
    }

    pub fn define(&mut self, name: String, params: Params, action: CommandAction) {
        self.dictionary.add(Command {
            name,
            params,
            action,
        })
    }

    pub fn read_token(&mut self) -> Result<Token, Box<dyn Error>> {
        self.consume_whitespace();
        let frame = self.get_top_frame();
        if frame.current_char() == '\0' {
            return Err(Box::from("reached end of file"));
        }
        if frame.current_char() == '(' {
            let token = self.read_parenthesis()?;
            let with_infix = self.handle_parse_infix(token);
            return Ok(with_infix);
        }
        let identifier = self.read_identifier()?;
        if let Some(command) = self.dictionary.lookup(&identifier) {
            let args = self.read_arguments(&command);
            let token = Token::Command(command, args);
            let with_infix = self.handle_parse_infix(token);
            Ok(with_infix)
        } else if identifier.starts_with(':') {
            let sanitized = identifier[1..].to_string();
            let token = Token::Variable(sanitized);
            let with_infix = self.handle_parse_infix(token);
            Ok(with_infix)
        } else if identifier.starts_with('\"') {
            let sanitized = identifier[1..].to_string();
            let token = Token::Word(sanitized);
            let with_infix = self.handle_parse_infix(token);
            Ok(with_infix)
        } else if let Ok(num) = identifier.parse::<f32>() {
            let token = Token::Number(num);
            let with_infix = self.handle_parse_infix(token);
            Ok(with_infix)
        } else if identifier == "true" || identifier == "false" {
            let token = Token::Boolean(identifier == "true");
            let with_infix = self.handle_parse_infix(token);
            Ok(with_infix)
        } else if identifier.starts_with('[') {
            let sanitized = identifier[1..identifier.len() - 1].to_string();
            let token = Token::List(sanitized);
            let with_infix = self.handle_parse_infix(token);
            Ok(with_infix)
        } else if identifier.ends_with(',') {
            let sanitized = identifier[..identifier.len() - 1].to_string();
            let token = Token::Command(Command::tto(), vec![Token::Word(sanitized)]);
            Ok(token)
        } else if identifier.ends_with("\'s") {
            let sanitized = identifier[..identifier.len() - 2].to_string();
            let command = Command::ask();
            let mut args = vec![Token::Word(sanitized)];
            let rest_args = self.read_fixed_arguments(&command.name, 1);
            args.extend(rest_args);
            let token = Token::Command(command, args);
            let with_infix = self.handle_parse_infix(token);
            Ok(with_infix)
        } else {
            Ok(Token::Undefined(identifier))
        }
    }

    fn consume_whitespace(&mut self) {
        let frame = self.get_top_frame();
        while frame.current_char().is_whitespace() && frame.current_char() != '\0' {
            frame.next();
        }
        if frame.current_char() == ';' {
            self.consume_until_newline();
        }
    }

    fn consume_until_newline(&mut self) {
        let frame = self.get_top_frame();
        while frame.current_char() != '\n' && frame.current_char() != '\0' {
            frame.next();
        }
        frame.next();
    }

    fn read_identifier(&mut self) -> Result<String, Box<dyn Error>> {
        self.consume_whitespace();
        let frame = self.get_top_frame();
        let mut command_name = String::new();
        let mut list_count = 0;
        while (!frame.current_char().is_whitespace() || list_count > 0)
            && frame.current_char() != '\0'
        {
            let chr = frame.current_char().clone();
            if chr == '[' {
                list_count += 1;
            } else if chr == ']' {
                list_count -= 1;
            }
            command_name.push(chr);
            frame.next();
        }
        Ok(command_name)
    }

    fn read_arguments(&mut self, command: &Command) -> Vec<Token> {
        match command.params {
            Params::Fixed(count) => self.read_fixed_arguments(&command.name, count),
            Params::Variadic(count) => self.read_variadic_arguments(count),
            Params::None => vec![],
        }
    }

    fn read_fixed_arguments(&mut self, command_name: &String, count: usize) -> Vec<Token> {
        let mut args = vec![];
        for _ in 0..count {
            if command_name == "to" {
                if let Ok(token) = self.read_procedure() {
                    args.push(token);
                }
            } else {
                if let Ok(token) = self.read_token() {
                    args.push(token);
                }
            }
        }
        args
    }

    fn read_variadic_arguments(&mut self, default_count: usize) -> Vec<Token> {
        let mut args = vec![];
        let frame = self.get_top_frame();
        if frame.in_paren {
            while let Ok(token) = self.read_token() {
                args.push(token);
            }
        } else {
            for _ in 0..default_count {
                if let Ok(token) = self.read_token() {
                    args.push(token);
                }
            }
        }
        args
    }

    fn read_parenthesis(&mut self) -> Result<Token, Box<dyn Error>> {
        let frame = self.get_top_frame();
        let mut code = String::new();
        if frame.current_char() == '(' {
            frame.next();
        }
        while frame.current_char() != ')' && frame.current_char() != '\0' {
            code.push(frame.current_char().clone());
            frame.next();
        }
        if frame.current_char() == '\0' {
            return Err(Box::from("found unclosed parenthesis"));
        }
        frame.next();
        Ok(Token::Command(Command::paren(), vec![Token::List(code)]))
    }

    fn handle_parse_infix(&mut self, token: Token) -> Token {
        if let Some(command) = self.peek_infix_operator() {
            return self.read_infix_operator(token, command);
        }
        token
    }

    fn peek_infix_operator(&mut self) -> Option<Command> {
        let saved_position = self.get_top_frame().position.clone();
        self.consume_whitespace();
        let frame = self.get_top_frame();
        let mut operator = String::new();
        while !frame.current_char().is_whitespace() && frame.current_char() != '\0' {
            operator.push(frame.current_char().clone());
            frame.next();
        }
        if let Some(command) = self.dictionary.lookup_infix(&operator) {
            Some(command)
        } else {
            self.get_top_frame().position = saved_position;
            None
        }
    }

    fn read_infix_operator(&mut self, left_arg: Token, operator: Command) -> Token {
        let mut args = self.read_fixed_arguments(&operator.name, 1);
        args.insert(0, left_arg);
        Token::Command(operator, args)
    }

    fn read_procedure(&mut self) -> Result<Token, Box<dyn Error>> {
        self.consume_whitespace();
        let frame = self.get_top_frame();
        let mut name = String::new();
        while frame.current_char().is_alphanumeric() {
            name.push(frame.current_char().clone());
            frame.next();
        }
        let mut params: Vec<String> = vec![];
        while frame.current_char() != '\n' {
            while frame.current_char() != ':'
                && frame.current_char() != '\n'
                && frame.current_char() != '\0'
            {
                frame.next();
            }
            frame.next();
            let mut param_name = String::new();
            while frame.current_char() != ' '
                && frame.current_char() != '\n'
                && frame.current_char() != '\0'
            {
                param_name.push(frame.current_char().clone());
                frame.next();
            }
            if !param_name.is_empty() {
                params.push(param_name);
            }
            while frame.current_char() == ' '
                && frame.current_char() != '\n'
                && frame.current_char() != '\0'
            {
                frame.next();
            }
        }
        self.consume_until_newline();
        let frame = self.get_top_frame();
        let mut block = String::new();
        while !block.ends_with("end\n") && frame.current_char() != '\0' {
            block.push(frame.current_char().clone());
            frame.next();
        }
        block = block.replacen("end\n", "", 1);
        Ok(Token::Procedure(name, params, block))
    }
}

#[derive(Debug, Clone)]
struct LexerFrame {
    text: String,
    position: usize,
    in_paren: bool,
}

impl LexerFrame {
    fn current_char(&self) -> char {
        self.text.chars().nth(self.position).unwrap_or('\0')
    }

    fn next(&mut self) -> char {
        let chr = self.current_char();
        self.position += 1;
        chr
    }
}
