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
        self.lexer.push_frame(code);
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
                    println!("error: {}", err);
                    let _ = self.ui_sender.send(UiEvent::Print(err.to_string()));
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
            Token::Variable(variable) => match self.datastore.get_variable(&variable) {
                Some(stored) => Ok(stored.clone()),
                None => Err(Box::from(format!("{} has no value", variable))),
            },
            Token::Undefined(undefined) => {
                Err(Box::from(format!("I don't know how to {}", undefined)))
            }
            other => Ok(other),
        }
    }

    pub fn execute_code_in_new_scope(
        &mut self,
        code: &str,
        local_params: Vec<(String, Token)>,
    ) -> Result<Token, Box<dyn Error>> {
        self.datastore.push_scope();
        for (param, arg) in &local_params {
            self.datastore.set_variable(param.clone(), arg.clone());
        }
        let return_value = self.interpret(code);
        for (param, _) in &local_params {
            self.datastore.remove_variable(&param);
        }
        self.datastore.pop_scope();
        return_value
    }

    pub fn define_procedure(&mut self, procedure: Procedure) {
        self.lexer.define(
            procedure.name.clone(),
            procedure.params.clone(),
            |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                if com.params.len() != args.len() {
                    return Err(Box::from("wrong number of inputs"));
                }
                let proc = int.datastore.get_procedure(&com.name).unwrap();
                let code = proc.code.clone();
                let mut local_params = vec![];
                for i in 0..com.params.len() {
                    local_params.push((com.params[i].clone(), args[i].clone()));
                }
                int.execute_code_in_new_scope(&code, local_params)
            },
        );
        self.datastore.set_procedure(procedure);
    }

    fn handle_input(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        match event {
            InputEvent::Interrupt => {
                self.clean_up();
                Err(Box::from("interrupt"))
            }
            InputEvent::Key(key) => {
                self.datastore.add_key_to_buffer(key);
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
        self.datastore.reset_scope();
        let _ = self.ui_sender.send(UiEvent::Done);
    }
}
