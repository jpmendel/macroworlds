use crate::interpreter::event::{InputEvent, UiEvent};
use crate::interpreter::lexer::Lexer;
use crate::language::command::{Params, Procedure};
use crate::language::dictionary::CommandDictionary;
use crate::language::token::Token;
use crate::state::state::State;
use crate::DEBUG;
use std::error::Error;
use std::sync::mpsc;

pub struct Interpreter {
    pub lexer: Lexer,
    pub state: State,
    pub ui_sender: mpsc::Sender<UiEvent>,
    pub input_receiver: mpsc::Receiver<InputEvent>,
}

impl Interpreter {
    pub fn new(
        ui_sender: mpsc::Sender<UiEvent>,
        input_receiver: mpsc::Receiver<InputEvent>,
    ) -> Self {
        let dictionary = CommandDictionary::default();
        let lexer = Lexer::with(dictionary);
        let state = State::new();
        Interpreter {
            lexer,
            state,
            ui_sender,
            input_receiver,
        }
    }

    pub fn interpret(&mut self, code: &str) -> Result<Token, Box<dyn Error>> {
        self.execute_code(code, false)
    }

    pub fn interpret_in_new_scope(
        &mut self,
        code: &str,
        local_params: Vec<(String, Token)>,
    ) -> Result<Token, Box<dyn Error>> {
        self.state.push_scope();
        for (param, arg) in &local_params {
            self.state.set_variable(param.clone(), arg.clone());
        }
        let return_value = self.interpret(code);
        for (param, _) in &local_params {
            self.state.remove_variable(&param);
        }
        self.state.pop_scope();
        return_value
    }

    pub fn interpret_in_parenthesis(&mut self, code: &str) -> Result<Token, Box<dyn Error>> {
        self.execute_code(code, true)
    }

    fn execute_code(&mut self, code: &str, in_paren: bool) -> Result<Token, Box<dyn Error>> {
        if code.is_empty() {
            self.exit_scope();
            return Ok(Token::Void);
        }
        self.lexer.push_frame(code, in_paren);
        loop {
            while let Ok(input_event) = self.input_receiver.try_recv() {
                self.handle_input(input_event)?;
            }
            let token = match self.lexer.read_token() {
                Ok(token) => token,
                Err(err) => {
                    if err.to_string() == "reached end of file" {
                        self.exit_scope();
                        break;
                    }
                    self.clean_up();
                    return Err(err);
                }
            };
            let mut is_return = false;
            if let Token::Command(command, _) = &token {
                is_return = command.name == "op";
            }
            match self.execute_command(token) {
                Ok(token) => {
                    if is_return {
                        self.exit_scope();
                        return Ok(token);
                    }
                }
                Err(err) => {
                    if err.to_string() == "interrupt" {
                        println!("Program Ended");
                    } else {
                        println!("error: {}", err);
                        let _ = self.ui_sender.send(UiEvent::Print(err.to_string()));
                    }
                    self.clean_up();
                    return Err(err);
                }
            }
        }
        Ok(Token::Void)
    }

    fn execute_command(&mut self, token: Token) -> Result<Token, Box<dyn Error>> {
        match token {
            Token::Command(command, args) => {
                let mut results = vec![];
                for arg in args {
                    let result = self.execute_command(arg)?;
                    results.push(result);
                }
                if DEBUG {
                    println!("{} {:?}", command.name, results);
                }
                (command.action)(self, &command.name, results)
            }
            Token::Variable(variable) => match self.state.get_variable(&variable) {
                Some(stored) => Ok(stored.clone()),
                None => Err(Box::from(format!("{} has no value", variable))),
            },
            Token::Undefined(undefined) => {
                Err(Box::from(format!("I don't know how to {}", undefined)))
            }
            other => Ok(other),
        }
    }

    pub fn define_procedure(&mut self, procedure: Procedure) {
        self.lexer.define(
            procedure.name.clone(),
            Params::Fixed(procedure.params.len()),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let proc = int.state.get_procedure(com).unwrap();
                if proc.params.len() != args.len() {
                    return Err(Box::from("wrong number of inputs"));
                }
                let code = proc.code.clone();
                let mut local_params = vec![];
                for i in 0..proc.params.len() {
                    local_params.push((proc.params[i].clone(), args[i].clone()));
                }
                int.interpret_in_new_scope(&code, local_params)
            },
        );
        self.state.set_procedure(procedure);
    }

    pub fn parse_list(&mut self, list: &String) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut items = vec![];
        let mut current_item = String::new();
        let mut reading_list = false;

        for chr in list.chars() {
            if chr == '[' {
                reading_list = true;
            } else if chr == ']' {
                reading_list = false;
                let token = Token::List(current_item.clone());
                items.push(token);
                current_item = String::new();
            } else if reading_list {
                current_item.push(chr);
            } else if !chr.is_whitespace() {
                current_item.push(chr);
            } else if !current_item.is_empty() {
                let token = self.parse_list_token(current_item)?;
                items.push(token);
                current_item = String::new();
            }
        }
        if !current_item.is_empty() {
            let token = self.parse_list_token(current_item)?;
            items.push(token);
        }
        Ok(items)
    }

    fn parse_list_token(&self, text: String) -> Result<Token, Box<dyn Error>> {
        if text.starts_with(':') {
            let var_name = text[1..].to_string();
            if let Some(var) = self.state.get_variable(&var_name) {
                Ok(var.clone())
            } else {
                Err(Box::from(format!("{} has no value", var_name)))
            }
        } else {
            Ok(Token::Word(text.clone()))
        }
    }

    fn handle_input(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::Interrupt => Err(Box::from("interrupt")),
            InputEvent::Key(key) => {
                self.state.add_key_to_buffer(key);
                Ok(())
            }
        }
    }

    fn exit_scope(&mut self) {
        let exited_main = self.lexer.pop_frame();
        if exited_main {
            println!("Done!");
            self.clean_up();
        }
    }

    fn clean_up(&mut self) {
        self.lexer.clear_frames();
        self.state.reset_scope();
        let _ = self.ui_sender.send(UiEvent::Done);
    }
}
