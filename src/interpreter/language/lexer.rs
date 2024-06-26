use crate::interpreter::language::language::Language;
use crate::interpreter::language::structure::{Command, CommandAction, Params};
use crate::interpreter::language::token::Token;
use crate::interpreter::util::error::eof_error;
use std::collections::VecDeque;
use std::error::Error;

pub struct Lexer {
    language: Language,
    code_blocks: VecDeque<CodeBlock>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            language: Language::default(),
            code_blocks: VecDeque::new(),
        }
    }

    pub fn push_block(&mut self, text: &str, in_paren: bool) {
        let block = CodeBlock {
            text: String::from(text),
            position: 0,
            in_paren,
        };
        self.code_blocks.push_back(block);
    }

    pub fn pop_block(&mut self) -> bool {
        let exiting_main = self.code_blocks.len() == 1;
        if self.code_blocks.len() > 0 {
            self.code_blocks.pop_back();
        }
        exiting_main
    }

    pub fn clear_blocks(&mut self) {
        self.code_blocks.clear();
    }

    pub fn return_to_start_of_block(&mut self) {
        let block = self.code_blocks.back_mut().unwrap();
        block.position = 0;
    }

    fn current_block(&mut self) -> &mut CodeBlock {
        self.code_blocks.back_mut().unwrap()
    }

    pub fn define(
        &mut self,
        name: &str,
        params: Params,
        action: CommandAction,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(command) = self.language.lookup(&name) {
            if command.is_reserved {
                return Err(Box::from(format!("{} is reserved", name)));
            }
        }
        let new_command = Command::user_defined(name, params, action);
        self.language.add(new_command);
        Ok(())
    }

    pub fn read_token(&mut self) -> Result<Token, Box<dyn Error>> {
        self.consume_whitespace();
        let block = self.current_block();
        if block.current_char() == '\0' {
            return Err(eof_error());
        }
        // Check for Comments
        if block.current_char() == ';' {
            self.consume_until_newline();
            return Ok(Token::Void);
        }
        // Check for Groups in Parenthesis
        if block.current_char() == '(' {
            let token = self.read_parenthesis()?;
            let with_infix = self.handle_parse_infix(token);
            return Ok(with_infix);
        }
        // Main Read
        let identifier = self.read_identifier()?;
        let token: Token;
        if let Some(command) = self.language.lookup(&identifier) {
            // Command
            let args = self.read_arguments(&command);
            token = Token::Command(command, args);
        } else if identifier.starts_with(':') {
            // Variable
            let sanitized = identifier[1..].to_string();
            token = Token::Variable(sanitized);
        } else if identifier.starts_with('\"') {
            // Word
            let sanitized = identifier[1..].to_string();
            token = Token::Word(sanitized);
        } else if let Ok(num) = identifier.parse::<f32>() {
            // Number
            token = Token::Number(num);
        } else if identifier == "true" || identifier == "false" {
            // Boolean
            token = Token::Boolean(identifier == "true");
        } else if identifier.starts_with('[') {
            // List
            let sanitized = identifier[1..identifier.len() - 1].to_string();
            token = Token::List(sanitized);
        } else if identifier.ends_with(',') {
            // Object "talkto" Command Shortcut
            let sanitized = identifier[..identifier.len() - 1].to_string();
            let token = Token::Command(Command::talkto(), vec![Token::Word(sanitized)]);
            return Ok(token);
        } else if identifier.ends_with("\'s") {
            // Object "ask" Command Shortcut
            let sanitized = identifier[..identifier.len() - 2].to_string();
            let command = Command::ask();
            let mut args = vec![Token::Word(sanitized)];
            let rest_args = self.read_fixed_arguments(&command.name, 1);
            args.extend(rest_args);
            token = Token::Command(command, args);
        } else if identifier.is_empty() {
            return Err(eof_error());
        } else {
            return Ok(Token::Undefined(identifier));
        }

        // Look for an infix operator like +, -, * or / which will come
        // after the first argument but before the second.
        let with_infix = self.handle_parse_infix(token);
        Ok(with_infix)
    }

    fn consume_whitespace(&mut self) {
        let block = self.current_block();
        while block.current_char().is_whitespace() && block.current_char() != '\0' {
            block.next();
        }
    }

    fn consume_until_newline(&mut self) {
        let block = self.current_block();
        while block.current_char() != '\n' && block.current_char() != '\0' {
            block.next();
        }
        block.next();
    }

    fn read_identifier(&mut self) -> Result<String, Box<dyn Error>> {
        self.consume_whitespace();
        let block = self.current_block();
        let mut identifier = String::new();
        let mut bracket_count = 0;
        let mut allow_whitespace = false;
        while (!block.current_char().is_whitespace() || bracket_count != 0 || allow_whitespace)
            && block.current_char() != '\0'
        {
            let chr = block.current_char().clone();
            if chr == '[' {
                bracket_count += 1;
            } else if chr == ']' {
                bracket_count -= 1;
            }
            if chr == '|' && bracket_count == 0 {
                allow_whitespace = !allow_whitespace;
                block.next();
                continue;
            }
            identifier.push(chr);
            block.next();
        }
        if bracket_count != 0 {
            return Err(Box::from("found unmatched brackets"));
        }
        if allow_whitespace {
            return Err(Box::from("found unmatched pipes"));
        }
        Ok(identifier)
    }

    fn read_arguments(&mut self, command: &Command) -> Vec<Token> {
        match command.params {
            Params::Fixed(count) => self.read_fixed_arguments(&command.name, count),
            Params::Variadic(count) => self.read_variadic_arguments(count),
            Params::None => vec![],
        }
    }

    fn read_fixed_arguments(&mut self, command_name: &str, count: usize) -> Vec<Token> {
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
        let block = self.current_block();
        if block.in_paren {
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
        let block = self.current_block();
        let mut code = String::new();
        let mut paren_count = 0;
        if block.current_char() == '(' {
            paren_count += 1;
            block.next();
        }
        while paren_count != 0 && block.current_char() != '\0' {
            if block.current_char() == '(' {
                paren_count += 1;
            } else if block.current_char() == ')' {
                paren_count -= 1;
            }
            // If we get matched parenthesis, exit early so we
            // don't add the closing paren to the code.
            if paren_count == 0 {
                break;
            }
            code.push(block.current_char().clone());
            block.next();
        }
        if paren_count != 0 {
            return Err(Box::from("found unmatched parenthesis"));
        }
        block.next();
        Ok(Token::Command(Command::paren(), vec![Token::List(code)]))
    }

    fn handle_parse_infix(&mut self, token: Token) -> Token {
        if let Some(command) = self.peek_infix_operator() {
            return self.read_infix_operator(token, command);
        }
        token
    }

    fn peek_infix_operator(&mut self) -> Option<Command> {
        let saved_position = self.current_block().position.clone();
        self.consume_whitespace();
        let block = self.current_block();
        let mut operator = String::new();
        while !block.current_char().is_whitespace() && block.current_char() != '\0' {
            operator.push(block.current_char().clone());
            block.next();
        }
        if let Some(command) = self.language.lookup_infix(&operator) {
            Some(command)
        } else {
            self.current_block().position = saved_position;
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
        let block = self.current_block();

        // Read procedure name.
        let mut name = String::new();
        while block.current_char().is_alphanumeric() {
            name.push(block.current_char().clone());
            block.next();
        }

        // Read parameter names.
        let mut params: Vec<String> = vec![];
        while block.current_char() != '\n' {
            while block.current_char() != ':'
                && block.current_char() != '\n'
                && block.current_char() != '\0'
            {
                block.next();
            }
            block.next();
            let mut param_name = String::new();
            while block.current_char() != ' '
                && block.current_char() != '\n'
                && block.current_char() != '\0'
            {
                param_name.push(block.current_char().clone());
                block.next();
            }
            if !param_name.is_empty() {
                params.push(param_name);
            }
            while block.current_char() == ' '
                && block.current_char() != '\n'
                && block.current_char() != '\0'
            {
                block.next();
            }
        }

        // Read the code block.
        self.consume_until_newline();
        let mut code = String::new();
        let mut code_line = String::new();
        let mut is_proc_complete = false;
        let block = self.current_block();
        while block.current_char() != '\0' {
            while block.current_char().is_whitespace() && block.current_char() != '\0' {
                block.next();
            }
            while block.current_char() != '\n' && block.current_char() != '\0' {
                code_line.push(block.current_char().clone());
                block.next();
            }

            // Include the newline.
            code_line.push(block.current_char().clone());
            block.next();

            // Find the "end" keyword to know when to stop.
            if code_line == "end\n" {
                is_proc_complete = true;
                break;
            }

            // Add the line to the code and continue.
            code += &code_line;
            code_line = String::new();
        }
        if !is_proc_complete {
            return Err(Box::from(format!("procedure {} has no end", name)));
        }
        Ok(Token::Procedure(name, params, code))
    }
}

#[derive(Debug, Clone)]
struct CodeBlock {
    text: String,
    position: usize,
    in_paren: bool,
}

impl CodeBlock {
    fn current_char(&self) -> char {
        self.text.chars().nth(self.position).unwrap_or('\0')
    }

    fn next(&mut self) {
        self.position += 1;
    }
}
