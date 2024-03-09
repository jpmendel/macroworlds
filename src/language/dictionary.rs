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

        // General Commands
        dictionary.add(Command::abs());
        dictionary.add(Command::and());
        dictionary.add(Command::ascii());
        dictionary.add(Command::ask());
        dictionary.add(Command::bk());
        dictionary.add(Command::butfirst());
        dictionary.add(Command::butlast());
        dictionary.add(Command::cc());
        dictionary.add(Command::cg());
        dictionary.add(Command::clean());
        dictionary.add(Command::color());
        dictionary.add(Command::ct());
        dictionary.add(Command::difference());
        dictionary.add(Command::dolist());
        dictionary.add(Command::dotimes());
        dictionary.add(Command::empty());
        dictionary.add(Command::equal());
        dictionary.add(Command::fd());
        dictionary.add(Command::first());
        dictionary.add(Command::forever());
        dictionary.add(Command::fput());
        dictionary.add(Command::greater());
        dictionary.add(Command::heading());
        dictionary.add(Command::hidetext());
        dictionary.add(Command::home());
        dictionary.add(Command::ht());
        dictionary.add(Command::ifelse());
        dictionary.add(Command::ifthen());
        dictionary.add(Command::item());
        dictionary.add(Command::key());
        dictionary.add(Command::last());
        dictionary.add(Command::less());
        dictionary.add(Command::list());
        dictionary.add(Command::local());
        dictionary.add(Command::lput());
        dictionary.add(Command::lt());
        dictionary.add(Command::make());
        dictionary.add(Command::member());
        dictionary.add(Command::minus());
        dictionary.add(Command::newtext());
        dictionary.add(Command::newturtle());
        dictionary.add(Command::not());
        dictionary.add(Command::op());
        dictionary.add(Command::or());
        dictionary.add(Command::pd());
        dictionary.add(Command::pi());
        dictionary.add(Command::pick());
        dictionary.add(Command::pos());
        dictionary.add(Command::power());
        dictionary.add(Command::print());
        dictionary.add(Command::product());
        dictionary.add(Command::pu());
        dictionary.add(Command::quotient());
        dictionary.add(Command::random());
        dictionary.add(Command::readchar());
        dictionary.add(Command::remove());
        dictionary.add(Command::repeat());
        dictionary.add(Command::rt());
        dictionary.add(Command::setc());
        dictionary.add(Command::setfontsize());
        dictionary.add(Command::seth());
        dictionary.add(Command::setpos());
        dictionary.add(Command::settc());
        dictionary.add(Command::setx());
        dictionary.add(Command::sety());
        dictionary.add(Command::show());
        dictionary.add(Command::showtext());
        dictionary.add(Command::st());
        dictionary.add(Command::sum());
        dictionary.add(Command::to());
        dictionary.add(Command::tto());
        dictionary.add(Command::wait());
        dictionary.add(Command::who());
        dictionary.add(Command::word());
        dictionary.add(Command::xcor());
        dictionary.add(Command::ycor());

        // Hidden Commands
        dictionary.add(Command::paren());
        dictionary.add(Command::loopback());

        // Infix Operators
        dictionary.add_infix(String::from("+"), Command::sum());
        dictionary.add_infix(String::from("-"), Command::difference());
        dictionary.add_infix(String::from("*"), Command::product());
        dictionary.add_infix(String::from("/"), Command::quotient());
        dictionary.add_infix(String::from("^"), Command::power());
        dictionary.add_infix(String::from("="), Command::equal());
        dictionary.add_infix(String::from(">"), Command::greater());
        dictionary.add_infix(String::from("<"), Command::less());

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
