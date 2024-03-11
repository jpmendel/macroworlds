use crate::interpreter::event::{EventHandler, InputEvent, UiEvent};
use crate::interpreter::event::{UiContext, UiEventHandler};
use crate::interpreter::lexer::Lexer;
use crate::language::command::{Params, Procedure};
use crate::language::token::Token;
use crate::language::util::decode_token;
use crate::state::state::State;
use crate::DEBUG;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};

pub struct Interpreter {
    pub lexer: Lexer,
    pub state: State,
    pub event: EventHandler,
}

impl Interpreter {
    pub fn new(input_receiver: mpsc::Receiver<InputEvent>) -> Self {
        Interpreter {
            lexer: Lexer::new(),
            state: State::new(),
            event: EventHandler::new(input_receiver),
        }
    }

    pub fn interpret_main(&mut self, code: &str) {
        self.state.reset_timer();
        let _ = self.execute_code(code, false, true);
    }

    pub fn interpret(&mut self, code: &str) -> Result<Token, Box<dyn Error>> {
        self.execute_code(code, false, false)
    }

    pub fn interpret_in_parenthesis(&mut self, code: &str) -> Result<Token, Box<dyn Error>> {
        self.execute_code(code, true, false)
    }

    pub fn interpret_in_new_scope(
        &mut self,
        code: &str,
        local_params: Vec<(String, Token)>,
    ) -> Result<Token, Box<dyn Error>> {
        self.state.data.push_scope();
        for (param, arg) in &local_params {
            self.state.data.set_variable(param.clone(), arg.clone());
        }
        let return_value = self.interpret(code);
        for (param, _) in &local_params {
            self.state.data.remove_variable(&param);
        }
        self.state.data.pop_scope();
        return_value
    }

    fn execute_code(
        &mut self,
        code: &str,
        in_paren: bool,
        handle_error: bool,
    ) -> Result<Token, Box<dyn Error>> {
        if code.is_empty() {
            return Ok(Token::Void);
        }
        self.lexer.push_frame(code, in_paren);
        loop {
            while let Ok(input_event) = self.event.receive_input_event() {
                self.handle_input(input_event)?;
            }
            let token = match self.lexer.read_token() {
                Ok(token) => token,
                Err(err) => {
                    if err.to_string() == "eof" {
                        self.exit_scope();
                        break;
                    }
                    if handle_error {
                        println!("error: {}", err);
                        self.event
                            .send_ui_event(UiEvent::ConsolePrint(err.to_string()));
                        self.clean_up();
                    } else {
                        self.exit_scope();
                    }
                    return Err(err);
                }
            };
            let mut is_return = false;
            if let Token::Command(command, _) = &token {
                is_return = command.name == "output";
            }
            match self.execute_command(token) {
                Ok(token) => {
                    if is_return {
                        self.exit_scope();
                        return Ok(token);
                    }
                }
                Err(err) => {
                    if handle_error {
                        if err.to_string() == "interrupt" {
                            println!("Program Ended");
                        } else {
                            println!("error: {}", err);
                            self.event
                                .send_ui_event(UiEvent::ConsolePrint(err.to_string()));
                        }
                        self.clean_up();
                    } else {
                        self.exit_scope();
                    }
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
            Token::Variable(variable) => match self.state.data.get_variable(&variable) {
                Some(stored) => Ok(stored.clone()),
                None => Err(Box::from(format!("{} has no value", variable))),
            },
            Token::Undefined(undefined) => {
                Err(Box::from(format!("I don't know how to {}", undefined)))
            }
            other => Ok(other),
        }
    }

    pub fn define_procedure(&mut self, procedure: Procedure) -> Result<(), Box<dyn Error>> {
        self.lexer.define(
            procedure.name.clone(),
            Params::Fixed(procedure.params.len()),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let proc = int.state.data.get_procedure(com).unwrap();
                if proc.params.len() != args.len() {
                    return Err(Box::from(format!(
                        "{} expected {} inputs",
                        proc.name,
                        proc.params.len()
                    )));
                }
                let code = proc.code.clone();
                let mut local_params = vec![];
                for i in 0..proc.params.len() {
                    local_params.push((proc.params[i].clone(), args[i].clone()));
                }
                int.interpret_in_new_scope(&code, local_params)
            },
        )?;
        self.state.data.set_procedure(procedure);
        Ok(())
    }

    pub fn define_object_property(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        // Getter
        self.lexer.define(
            name.clone(),
            Params::None,
            |int: &mut Interpreter, com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                if let Some(value) = turtle.backpack.get(com) {
                    Ok(value.clone())
                } else {
                    Err(Box::from(format!("turtle does not own {}", com)))
                }
            },
        )?;

        // Setter
        self.lexer.define(
            format!("set{}", name.clone()),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let turtle = int.state.canvas.current_turtle()?;
                let item_name: String = com.chars().skip(3).collect();
                turtle.backpack.insert(item_name, token);
                Ok(Token::Void)
            },
        )?;

        // Add to Backpack
        self.state.canvas.init_backpack_property(name);
        Ok(())
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
            if let Some(var) = self.state.data.get_variable(&var_name) {
                Ok(var.clone())
            } else {
                Err(Box::from(format!("{} has no value", var_name)))
            }
        } else {
            Ok(Token::Word(text.clone()))
        }
    }

    pub fn bind_ui_handler(
        &mut self,
        handler: Arc<Mutex<dyn UiEventHandler>>,
        context: Arc<Mutex<dyn UiContext>>,
    ) {
        self.event.ui_handler = Some(handler);
        self.event.ui_context = Some(context);
    }

    pub fn clear_ui_handler(&mut self) {
        self.event.ui_handler = None;
        self.event.ui_context = None;
    }

    fn handle_input(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::Interrupt => Err(Box::from("interrupt")),
            InputEvent::Key(key) => {
                self.state.input.add_key_to_buffer(key);
                Ok(())
            }
        }
    }

    pub fn clear_input_events(&self) {
        while self.event.input_receiver.try_recv().is_ok() {
            // Consume remaining events.
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
        self.state.data.reset_scope();
        self.event.send_ui_event(UiEvent::Done);
    }

    pub fn reset(&mut self) {
        self.lexer = Lexer::new();
        self.state = State::new();
    }
}
