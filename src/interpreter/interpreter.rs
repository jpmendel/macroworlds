use crate::interpreter::datastore::Datastore;
use crate::interpreter::lexer::Lexer;
use crate::language::command::{Command, Procedure};
use crate::language::dictionary::CommandDictionary;
use crate::language::event::{InputEvent, UiEvent};
use crate::language::token::Token;
use crate::DEBUG;
use std::error::Error;
use std::sync::mpsc;

pub struct Interpreter {
    pub lexer: Lexer,
    pub datastore: Datastore,
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
        let datastore = Datastore::new();
        Interpreter {
            lexer,
            datastore,
            ui_sender,
            input_receiver,
        }
    }

    pub fn interpret(&mut self, code: &str) -> Result<Token, Box<dyn Error>> {
        if code.is_empty() {
            self.exit_scope();
            return Ok(Token::Void);
        }
        self.lexer.load(code);
        loop {
            if let Ok(input_event) = self.input_receiver.try_recv() {
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
                    println!("error: {}", err);
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
                (command.action)(self, &command, results)
            }
            Token::Variable(variable) => {
                let var_name = variable.replacen(':', "", 1);
                match self.datastore.get_variable(&var_name) {
                    Some(stored) => Ok(stored.clone()),
                    None => Err(Box::from(format!("{} has no value", var_name))),
                }
            }
            Token::Undefined(undefined) => {
                Err(Box::from(format!("I don't know how to {}", undefined)))
            }
            other => Ok(other),
        }
    }

    pub fn define_procedure(&mut self, procedure: Procedure) {
        self.lexer.define(
            procedure.name.clone(),
            procedure.params.clone(),
            |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                if com.params.len() != args.len() {
                    return Err(Box::from("wrong number of inputs"));
                }
                int.datastore.push_scope();
                for i in 0..com.params.len() {
                    let arg_name = com.params.get(i).unwrap();
                    let value = args.get(i).unwrap();
                    int.datastore.set_variable(arg_name.clone(), value.clone());
                }
                let return_value = int.execute_procedure(com.name.clone());
                for param in &com.params {
                    int.datastore.remove_variable(&param);
                }
                int.datastore.pop_scope();
                return_value
            },
        );
        self.datastore.set_procedure(procedure);
    }

    pub fn execute_procedure(&mut self, name: String) -> Result<Token, Box<dyn Error>> {
        if let Some(proc) = self.datastore.get_procedure(&name) {
            return self.interpret(&proc.code.clone());
        }
        Ok(Token::Void)
    }

    pub fn emit_ui_event(&self, event: UiEvent) {
        self.ui_sender.send(event).unwrap_or(());
    }

    fn handle_input(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::Interrupt => {
                self.clean_up();
                Err(Box::from("interrupt"))
            }
            _ => Ok(()),
        }
    }

    fn exit_scope(&mut self) {
        let exited_main = self.lexer.pop_frame();
        if exited_main {
            println!("Done!");
            self.emit_ui_event(UiEvent::Done);
        }
    }

    fn clean_up(&mut self) {
        self.lexer.clear_frames();
        self.emit_ui_event(UiEvent::Done);
    }
}
