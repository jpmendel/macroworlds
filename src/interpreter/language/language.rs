use crate::interpreter::language::command::command::Command;
use std::collections::HashMap;

pub struct Language {
    commands: HashMap<Box<str>, Command>,
    infix_operators: HashMap<Box<str>, Command>,
}

impl Language {
    pub fn default() -> Self {
        let mut language = Language {
            commands: HashMap::new(),
            infix_operators: HashMap::new(),
        };

        // General Commands
        language.add(Command::abs());
        language.add(Command::again());
        language.add(Command::and());
        language.add(Command::announce());
        language.add(Command::arctan());
        language.add(Command::ascii());
        language.add(Command::ask());
        language.add(Command::back());
        language.add(Command::bg());
        language.add(Command::butfirst());
        language.add(Command::butlast());
        language.add(Command::carefully());
        language.add(Command::cc());
        language.add(Command::cg());
        language.add(Command::char());
        language.add(Command::chdir());
        language.add(Command::clean());
        language.add(Command::clearname());
        language.add(Command::clearnames());
        language.add(Command::cleartext());
        language.add(Command::clicked());
        language.add(Command::color());
        language.add(Command::colorunder());
        language.add(Command::cos());
        language.add(Command::count());
        language.add(Command::currentdir());
        language.add(Command::difference());
        language.add(Command::directories());
        language.add(Command::distance());
        language.add(Command::dolist());
        language.add(Command::dotimes());
        language.add(Command::empty());
        language.add(Command::equal());
        language.add(Command::errormessage());
        language.add(Command::exp());
        language.add(Command::files());
        language.add(Command::first());
        language.add(Command::fontsize());
        language.add(Command::forever());
        language.add(Command::forward());
        language.add(Command::fput());
        language.add(Command::freeze());
        language.add(Command::greater());
        language.add(Command::heading());
        language.add(Command::home());
        language.add(Command::ht());
        language.add(Command::ifelse());
        language.add(Command::ifthen());
        language.add(Command::int());
        language.add(Command::islist());
        language.add(Command::isnumber());
        language.add(Command::isword());
        language.add(Command::item());
        language.add(Command::key());
        language.add(Command::keydown());
        language.add(Command::last());
        language.add(Command::left());
        language.add(Command::less());
        language.add(Command::letvar());
        language.add(Command::list());
        language.add(Command::ln());
        language.add(Command::loadpict());
        language.add(Command::loadshape());
        language.add(Command::loadtext());
        language.add(Command::local());
        language.add(Command::log());
        language.add(Command::lput());
        language.add(Command::make());
        language.add(Command::member());
        language.add(Command::minus());
        language.add(Command::newtext());
        language.add(Command::newturtle());
        language.add(Command::not());
        language.add(Command::or());
        language.add(Command::output());
        language.add(Command::pd());
        language.add(Command::pensize());
        language.add(Command::pi());
        language.add(Command::pick());
        language.add(Command::pictlist());
        language.add(Command::pos());
        language.add(Command::power());
        language.add(Command::print());
        language.add(Command::procedures());
        language.add(Command::product());
        language.add(Command::projectsize());
        language.add(Command::pu());
        language.add(Command::quotient());
        language.add(Command::random());
        language.add(Command::readchar());
        language.add(Command::readclick());
        language.add(Command::remainder());
        language.add(Command::remove());
        language.add(Command::repeat());
        language.add(Command::resett());
        language.add(Command::right());
        language.add(Command::round());
        language.add(Command::run());
        language.add(Command::setbg());
        language.add(Command::setcolor());
        language.add(Command::setfontsize());
        language.add(Command::setheading());
        language.add(Command::setpensize());
        language.add(Command::setpos());
        language.add(Command::setprojectsize());
        language.add(Command::setshape());
        language.add(Command::setsize());
        language.add(Command::setstyle());
        language.add(Command::setx());
        language.add(Command::sety());
        language.add(Command::shape());
        language.add(Command::show());
        language.add(Command::sin());
        language.add(Command::size());
        language.add(Command::sqrt());
        language.add(Command::st());
        language.add(Command::sum());
        language.add(Command::talkto());
        language.add(Command::tan());
        language.add(Command::text());
        language.add(Command::textlist());
        language.add(Command::timer());
        language.add(Command::to());
        language.add(Command::touching());
        language.add(Command::towards());
        language.add(Command::turtlesown());
        language.add(Command::unfreeze());
        language.add(Command::visible());
        language.add(Command::wait());
        language.add(Command::who());
        language.add(Command::word());
        language.add(Command::xcor());
        language.add(Command::ycor());

        // Alias
        language.add_alias("bf", Command::butfirst());
        language.add_alias("bk", Command::back());
        language.add_alias("bl", Command::butlast());
        language.add_alias("ct", Command::cleartext());
        language.add_alias("fd", Command::forward());
        language.add_alias("lt", Command::left());
        language.add_alias("op", Command::output());
        language.add_alias("pr", Command::print());
        language.add_alias("rt", Command::right());
        language.add_alias("se", Command::list());
        language.add_alias("sentence", Command::list());
        language.add_alias("setc", Command::setcolor());
        language.add_alias("seth", Command::setheading());
        language.add_alias("setsh", Command::setshape());
        language.add_alias("tto", Command::talkto());

        // Infix Operators
        language.add_infix("+", Command::sum());
        language.add_infix("-", Command::difference());
        language.add_infix("*", Command::product());
        language.add_infix("/", Command::quotient());
        language.add_infix("^", Command::power());
        language.add_infix("%", Command::remainder());
        language.add_infix("=", Command::equal());
        language.add_infix(">", Command::greater());
        language.add_infix("<", Command::less());

        // Hidden Commands
        language.add(Command::paren());

        language
    }

    pub fn lookup(&self, command_name: &str) -> Option<Command> {
        if let Some(command) = self.commands.get(command_name) {
            return Some(command.clone());
        }
        None
    }

    pub fn add(&mut self, command: Command) {
        self.commands.insert(command.name.clone(), command);
    }

    pub fn add_alias(&mut self, alias: &str, command: Command) {
        self.commands.insert(Box::from(alias), command);
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
