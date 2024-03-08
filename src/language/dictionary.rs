use crate::language::command::Command;
use std::collections::HashMap;

pub struct CommandDictionary {
    functions: HashMap<String, Command>,
    infix_operators: HashMap<String, Command>,
}

impl CommandDictionary {
    pub fn default() -> Self {
        let mut dictionary = CommandDictionary {
            functions: HashMap::new(),
            infix_operators: HashMap::new(),
        };
        dictionary.add(Command::add());
        dictionary.add(Command::addr());
        dictionary.add(Command::and());
        dictionary.add(Command::ask());
        dictionary.add(Command::bk());
        dictionary.add(Command::clean());
        dictionary.add(Command::div());
        dictionary.add(Command::eq());
        dictionary.add(Command::fd());
        dictionary.add(Command::forever());
        dictionary.add(Command::geq());
        dictionary.add(Command::greater());
        dictionary.add(Command::ht());
        dictionary.add(Command::ifelse());
        dictionary.add(Command::ifthen());
        dictionary.add(Command::leq());
        dictionary.add(Command::less());
        dictionary.add(Command::local());
        dictionary.add(Command::lt());
        dictionary.add(Command::loopback());
        dictionary.add(Command::make());
        dictionary.add(Command::mul());
        dictionary.add(Command::ne());
        dictionary.add(Command::newturtle());
        dictionary.add(Command::not());
        dictionary.add(Command::op());
        dictionary.add(Command::or());
        dictionary.add(Command::paren());
        dictionary.add(Command::pd());
        dictionary.add(Command::print());
        dictionary.add(Command::pu());
        dictionary.add(Command::readchar());
        dictionary.add(Command::repeat());
        dictionary.add(Command::rt());
        dictionary.add(Command::setx());
        dictionary.add(Command::sety());
        dictionary.add(Command::setpos());
        dictionary.add(Command::seth());
        dictionary.add(Command::setc());
        dictionary.add(Command::st());
        dictionary.add(Command::sub());
        dictionary.add(Command::to());
        dictionary.add(Command::wait());

        dictionary.add_infix(String::from("+"), Command::add());
        dictionary.add_infix(String::from("-"), Command::sub());
        dictionary.add_infix(String::from("*"), Command::mul());
        dictionary.add_infix(String::from("/"), Command::div());
        dictionary.add_infix(String::from("="), Command::eq());
        dictionary.add_infix(String::from("!="), Command::ne());
        dictionary.add_infix(String::from(">"), Command::greater());
        dictionary.add_infix(String::from("<"), Command::less());
        dictionary.add_infix(String::from(">="), Command::geq());
        dictionary.add_infix(String::from("<="), Command::leq());

        dictionary
    }

    pub fn lookup(&self, command_name: &String) -> Option<Command> {
        if let Some(command) = self.functions.get(command_name) {
            return Some(command.clone());
        }
        None
    }

    pub fn add(&mut self, command: Command) {
        self.functions.insert(command.name.clone(), command);
    }

    pub fn lookup_infix(&self, command_name: &String) -> Option<Command> {
        if let Some(command) = self.infix_operators.get(command_name) {
            return Some(command.clone());
        }
        None
    }

    pub fn add_infix(&mut self, name: String, command: Command) {
        self.infix_operators.insert(name, command);
    }
}
