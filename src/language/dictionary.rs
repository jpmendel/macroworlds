use crate::language::command::Command;
use std::collections::HashMap;

pub struct CommandDictionary {
    functions: HashMap<String, Command>,
    infix_operators: HashMap<String, Command>,
}

impl CommandDictionary {
    pub fn default() -> Self {
        let mut infix_operators = HashMap::new();
        infix_operators.insert(String::from("+"), Command::add());
        infix_operators.insert(String::from("-"), Command::sub());
        infix_operators.insert(String::from("*"), Command::mul());
        infix_operators.insert(String::from("/"), Command::div());
        infix_operators.insert(String::from("="), Command::eq());
        infix_operators.insert(String::from("!="), Command::ne());

        let mut dictionary = CommandDictionary {
            functions: HashMap::new(),
            infix_operators,
        };
        dictionary.add(Command::add());
        dictionary.add(Command::sub());
        dictionary.add(Command::mul());
        dictionary.add(Command::div());
        dictionary.add(Command::eq());
        dictionary.add(Command::ne());
        dictionary.add(Command::greater());
        dictionary.add(Command::less());
        dictionary.add(Command::geq());
        dictionary.add(Command::leq());
        dictionary.add(Command::not());
        dictionary.add(Command::fd());
        dictionary.add(Command::bk());
        dictionary.add(Command::lt());
        dictionary.add(Command::rt());
        dictionary.add(Command::setpos());
        dictionary.add(Command::seth());
        dictionary.add(Command::setc());
        dictionary.add(Command::pd());
        dictionary.add(Command::pu());
        dictionary.add(Command::st());
        dictionary.add(Command::ht());
        dictionary.add(Command::clean());
        dictionary.add(Command::newturtle());
        dictionary.add(Command::addr());
        dictionary.add(Command::wait());
        dictionary.add(Command::print());
        dictionary.add(Command::make());
        dictionary.add(Command::to());
        dictionary.add(Command::local());
        dictionary.add(Command::op());
        dictionary.add(Command::ifthen());
        dictionary.add(Command::ifelse());
        dictionary.add(Command::repeat());
        dictionary.add(Command::forever());
        dictionary.add(Command::loopback());
        dictionary.add(Command::readchar());
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
}
