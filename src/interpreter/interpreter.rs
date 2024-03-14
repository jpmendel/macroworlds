use crate::interpreter::event::{EventHandler, InputEvent, UiEvent};
use crate::interpreter::event::{UiContext, UiEventHandler};
use crate::interpreter::language::command::command::Params;
use crate::interpreter::language::procedure::Procedure;
use crate::interpreter::language::token::Token;
use crate::interpreter::language::util::decode_token;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::performance::PerformanceTracker;
use crate::interpreter::state::state::State;
use crate::interpreter::util::{is_eof, is_interrupt};
use crate::DEBUG;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};

#[cfg(feature = "performance")]
use std::time::Instant;

pub struct Interpreter {
    pub lexer: Lexer,
    pub state: State,
    pub event: EventHandler,
    pub performance: PerformanceTracker,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            lexer: Lexer::new(),
            state: State::new(),
            event: EventHandler::new(),
            performance: PerformanceTracker::new(),
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
        if self.state.data.reached_max_scope_depth() {
            return Err(Box::from("maximum stack depth exceeded"));
        }
        self.state.data.push_scope();
        for (param, arg) in &local_params {
            self.state.data.set_variable(param, arg.clone());
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
        self.lexer.push_block(code, in_paren);
        loop {
            while let Ok(input_event) = self.event.receive_input() {
                self.handle_input(input_event)?;
            }
            let token = match self.lexer.read_token() {
                Ok(token) => token,
                Err(err) => {
                    if is_eof(&err) {
                        self.exit_scope();
                        break;
                    }
                    if handle_error {
                        println!("error: {}", err);
                        self.event.send_ui(UiEvent::ConsolePrint(err.to_string()));
                        self.terminate_program();
                    } else {
                        self.exit_scope();
                    }
                    return Err(err);
                }
            };
            let is_return = if let Token::Command(command, _) = &token {
                command.is("output")
            } else {
                false
            };
            match self.execute_command(token) {
                Ok(token) => {
                    if is_return {
                        self.exit_scope();
                        return Ok(token);
                    }
                }
                Err(err) => {
                    if handle_error {
                        if is_interrupt(&err) {
                            println!("Program Ended");
                        } else {
                            println!("error: {}", err);
                            self.event.send_ui(UiEvent::ConsolePrint(err.to_string()));
                        }
                        self.terminate_program();
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
                let mut computed_args = vec![];
                for arg in args {
                    let computed_arg = self.execute_command(arg)?;
                    computed_args.push(computed_arg);
                }
                if DEBUG {
                    println!("{} {:?}", command.name, computed_args);
                }
                #[cfg(not(feature = "performance"))]
                {
                    (command.action)(self, &command.name, computed_args)
                }
                #[cfg(feature = "performance")]
                {
                    let start = Instant::now();
                    let result = (command.action)(self, &command.name, computed_args.clone())?;
                    let time = start.elapsed();
                    let mut tag = command.name.to_string();
                    for arg in computed_args {
                        tag += &format!(" {}", arg.to_string());
                    }
                    self.performance.record(&command.name, time, tag);
                    Ok(result)
                }
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
            &procedure.name,
            Params::Fixed(procedure.params.len()),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let proc = int.state.data.get_procedure(com).unwrap();
                if proc.params.len() != args.len() {
                    return Err(Box::from(format!(
                        "{} expected {} inputs",
                        proc.name,
                        proc.params.len()
                    )));
                }
                let mut local_params = vec![];
                for i in 0..proc.params.len() {
                    local_params.push((proc.params[i].clone(), args[i].clone()));
                }
                let code = proc.code.clone();
                int.interpret_in_new_scope(&code, local_params)
            },
        )?;
        self.state.data.set_procedure(procedure);
        Ok(())
    }

    pub fn define_object_property(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        // Getter
        self.lexer.define(
            name,
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
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
            &format!("set{}", name),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let item_name = com.chars().skip(3).collect::<String>().into_boxed_str();
                let turtle = int.state.canvas.current_turtle_mut()?;
                turtle.backpack.insert(item_name, token);
                Ok(Token::Void)
            },
        )?;

        // Add to Backpack
        self.state.canvas.init_backpack_property(name);
        Ok(())
    }

    pub fn parse_list(
        &mut self,
        list: &String,
        parse_numbers: bool,
    ) -> Result<Vec<Token>, Box<dyn Error>> {
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
                let token = self.parse_list_token(current_item, parse_numbers)?;
                items.push(token);
                current_item = String::new();
            }
        }
        if !current_item.is_empty() {
            let token = self.parse_list_token(current_item, parse_numbers)?;
            items.push(token);
        }
        Ok(items)
    }

    fn parse_list_token(&self, text: String, parse_numbers: bool) -> Result<Token, Box<dyn Error>> {
        if text.starts_with(':') {
            let var_name = text[1..].to_string();
            if let Some(var) = self.state.data.get_variable(&var_name) {
                return Ok(var.clone());
            } else {
                return Err(Box::from(format!("{} has no value", var_name)));
            }
        } else if parse_numbers {
            if let Ok(number) = text.parse::<f32>() {
                return Ok(Token::Number(number));
            }
        }
        Ok(Token::Word(text.clone()))
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

    pub fn bind_input_receiver(&mut self, receiver: mpsc::Receiver<InputEvent>) {
        self.event.input_receiver = Some(receiver);
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
        while self.event.receive_input().is_ok() {
            // Consume remaining events.
        }
    }

    fn exit_scope(&mut self) {
        let exited_main = self.lexer.pop_block();
        if exited_main {
            println!("Done!");
            self.terminate_program();
        }
    }

    fn terminate_program(&mut self) {
        self.lexer.clear_blocks();
        self.state.data.reset_scope();
        self.event.send_ui(UiEvent::Done);
    }

    pub fn reset(&mut self) {
        self.lexer = Lexer::new();
        self.state = State::new();
    }
}
