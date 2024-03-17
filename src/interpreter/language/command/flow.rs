use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::language::command::command::{Command, Params};
use crate::interpreter::language::token::Token;
use crate::interpreter::language::util::decode_boolean;
use crate::interpreter::language::util::{decode_list, decode_number};

impl Command {
    pub fn ifthen() -> Self {
        Command::reserved(
            "if",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let condition = decode_boolean(com, &args, 0)?;
                let true_code = decode_list(com, &args, 1)?;
                if condition {
                    int.interpret(&true_code)?;
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn ifelse() -> Self {
        Command::reserved(
            "ifelse",
            Params::Fixed(3),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let condition = decode_boolean(com, &args, 0)?;
                let true_code = decode_list(com, &args, 1)?;
                let false_code = decode_list(com, &args, 2)?;
                if condition {
                    int.interpret(&true_code)?;
                } else {
                    int.interpret(&false_code)?;
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn repeat() -> Self {
        Command::reserved(
            "repeat",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let count = decode_number(com, &args, 0)? as usize;
                let code = decode_list(com, &args, 1)?;
                for _ in 0..count {
                    int.interpret(&code)?;
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn forever() -> Self {
        Command::reserved(
            "forever",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let code = decode_list(com, &args, 0)?;
                loop {
                    int.interpret(&code)?;
                }
            },
        )
    }

    pub fn dotimes() -> Self {
        Command::reserved(
            "dotimes",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let loop_config = decode_list(com, &args, 0)?;
                let code = decode_list(com, &args, 1)?;
                let config_items = int.parse_list(&loop_config, true)?;
                let Some(Token::Word(var_name)) = config_items.get(0) else {
                    return Err(Box::from("dotimes expected a word for input 0 in input 0"));
                };
                let Some(Token::Number(count)) = config_items.get(1) else {
                    return Err(Box::from(
                        "dotimes expected a number for input 1 in input 0",
                    ));
                };
                for index in 0..(*count as usize) {
                    let local_params = vec![(var_name.clone(), Token::Number(index as f32))];
                    int.interpret_in_new_scope(&code, local_params)?;
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn dolist() -> Self {
        Command::reserved(
            "dolist",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let loop_config = decode_list(com, &args, 0)?;
                let code = decode_list(com, &args, 1)?;
                let config_items = int.parse_list(&loop_config, false)?;
                let Some(Token::Word(var_name)) = config_items.get(0) else {
                    return Err(Box::from("dolist expected a word for input 0 in input 0"));
                };
                let Some(Token::List(list)) = config_items.get(1) else {
                    return Err(Box::from("dolist expected a list for input 1 in input 0"));
                };
                let list_items = int.parse_list(&list, true)?;
                for item in list_items {
                    let local_params = vec![(var_name.clone(), item)];
                    int.interpret_in_new_scope(&code, local_params)?;
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn carefully() -> Self {
        Command::reserved(
            "carefully",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let check_code = decode_list(com, &args, 0)?;
                let error_code = decode_list(com, &args, 1)?;
                if let Err(err) = int.interpret(&check_code) {
                    int.state.data.set_last_error_message(err.to_string());
                    int.interpret(&error_code)?;
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn errormessage() -> Self {
        Command::reserved(
            "errormessage",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let error = int.state.data.get_last_error_message();
                Ok(Token::Word(error))
            },
        )
    }

    pub fn again() -> Self {
        Command::reserved(
            "again",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                int.lexer.return_to_start_of_block();
                Ok(Token::Void)
            },
        )
    }

    pub fn paren() -> Self {
        Command::reserved(
            "__paren",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let code = decode_list(com, &args, 0)?;
                let code_with_return = format!("op {}", code);
                int.interpret_in_parenthesis(&code_with_return)
            },
        )
    }
}
