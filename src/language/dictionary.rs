use crate::language::command::Command;
use std::collections::HashMap;

pub struct CommandDictionary {
    functions: HashMap<Box<str>, Command>,
    infix_operators: HashMap<Box<str>, Command>,
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
        dictionary.add(Command::announce());
        dictionary.add(Command::arctan());
        dictionary.add(Command::ascii());
        dictionary.add(Command::ask());
        dictionary.add(Command::back());
        dictionary.add(Command::bg());
        dictionary.add(Command::butfirst());
        dictionary.add(Command::butlast());
        dictionary.add(Command::carefully());
        dictionary.add(Command::cc());
        dictionary.add(Command::cg());
        dictionary.add(Command::char());
        dictionary.add(Command::clean());
        dictionary.add(Command::cleartext());
        dictionary.add(Command::color());
        dictionary.add(Command::colorunder());
        dictionary.add(Command::cos());
        dictionary.add(Command::count());
        dictionary.add(Command::difference());
        dictionary.add(Command::distance());
        dictionary.add(Command::dolist());
        dictionary.add(Command::dotimes());
        dictionary.add(Command::empty());
        dictionary.add(Command::equal());
        dictionary.add(Command::errormessage());
        dictionary.add(Command::first());
        dictionary.add(Command::fontsize());
        dictionary.add(Command::forever());
        dictionary.add(Command::forward());
        dictionary.add(Command::fput());
        dictionary.add(Command::greater());
        dictionary.add(Command::heading());
        dictionary.add(Command::hidetext());
        dictionary.add(Command::home());
        dictionary.add(Command::ht());
        dictionary.add(Command::ifelse());
        dictionary.add(Command::ifthen());
        dictionary.add(Command::int());
        dictionary.add(Command::islist());
        dictionary.add(Command::isnumber());
        dictionary.add(Command::isword());
        dictionary.add(Command::item());
        dictionary.add(Command::key());
        dictionary.add(Command::last());
        dictionary.add(Command::left());
        dictionary.add(Command::less());
        dictionary.add(Command::letvar());
        dictionary.add(Command::list());
        dictionary.add(Command::local());
        dictionary.add(Command::lput());
        dictionary.add(Command::make());
        dictionary.add(Command::member());
        dictionary.add(Command::minus());
        dictionary.add(Command::newprojectsize());
        dictionary.add(Command::newtext());
        dictionary.add(Command::newturtle());
        dictionary.add(Command::not());
        dictionary.add(Command::or());
        dictionary.add(Command::output());
        dictionary.add(Command::pd());
        dictionary.add(Command::pensize());
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
        dictionary.add(Command::recurse());
        dictionary.add(Command::remainder());
        dictionary.add(Command::remove());
        dictionary.add(Command::repeat());
        dictionary.add(Command::resett());
        dictionary.add(Command::right());
        dictionary.add(Command::round());
        dictionary.add(Command::setbg());
        dictionary.add(Command::setc());
        dictionary.add(Command::setfontsize());
        dictionary.add(Command::seth());
        dictionary.add(Command::setpensize());
        dictionary.add(Command::setpos());
        dictionary.add(Command::setsh());
        dictionary.add(Command::setx());
        dictionary.add(Command::sety());
        dictionary.add(Command::shape());
        dictionary.add(Command::show());
        dictionary.add(Command::showtext());
        dictionary.add(Command::sin());
        dictionary.add(Command::sqrt());
        dictionary.add(Command::st());
        dictionary.add(Command::sum());
        dictionary.add(Command::talkto());
        dictionary.add(Command::tan());
        dictionary.add(Command::timer());
        dictionary.add(Command::to());
        dictionary.add(Command::towards());
        dictionary.add(Command::turtlesown());
        dictionary.add(Command::wait());
        dictionary.add(Command::who());
        dictionary.add(Command::word());
        dictionary.add(Command::xcor());
        dictionary.add(Command::ycor());

        // Alias
        dictionary.add_alias("bf", Command::butfirst());
        dictionary.add_alias("bk", Command::back());
        dictionary.add_alias("bl", Command::butlast());
        dictionary.add_alias("ct", Command::cleartext());
        dictionary.add_alias("fd", Command::forward());
        dictionary.add_alias("lt", Command::left());
        dictionary.add_alias("op", Command::output());
        dictionary.add_alias("pr", Command::print());
        dictionary.add_alias("rt", Command::right());
        dictionary.add_alias("tto", Command::talkto());

        // Infix Operators
        dictionary.add_infix("+", Command::sum());
        dictionary.add_infix("-", Command::difference());
        dictionary.add_infix("*", Command::product());
        dictionary.add_infix("/", Command::quotient());
        dictionary.add_infix("^", Command::power());
        dictionary.add_infix("%", Command::remainder());
        dictionary.add_infix("=", Command::equal());
        dictionary.add_infix(">", Command::greater());
        dictionary.add_infix("<", Command::less());

        // Hidden Commands
        dictionary.add(Command::paren());

        dictionary
    }

    pub fn lookup(&self, command_name: &str) -> Option<Command> {
        if let Some(command) = self.functions.get(command_name) {
            return Some(command.clone());
        }
        None
    }

    pub fn add(&mut self, command: Command) {
        self.functions.insert(command.name.clone(), command);
    }

    pub fn add_alias(&mut self, alias: &str, command: Command) {
        self.functions.insert(Box::from(alias), command);
    }

    pub fn lookup_infix(&self, command_name: &str) -> Option<Command> {
        if let Some(command) = self.infix_operators.get(command_name) {
            return Some(command.clone());
        }
        None
    }

    pub fn add_infix(&mut self, name: &str, command: Command) {
        self.infix_operators.insert(Box::from(name), command);
    }
}
