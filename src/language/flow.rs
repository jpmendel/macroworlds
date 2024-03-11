use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params};
use crate::language::token::Token;
use crate::language::util::decode_boolean;
use crate::language::util::{decode_list, decode_number, join_to_list_string};

impl Command {
    pub fn ifthen() -> Self {
        Command::reserved(
            String::from("if"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("ifelse"),
            Params::Fixed(3),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("repeat"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let count = decode_number(com, &args, 0)? as usize;
                let code = decode_list(com, &args, 1)?;
                for _ in 0..count {
                    int.interpret_in_new_scope(&code, vec![])?;
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn forever() -> Self {
        Command::reserved(
            String::from("forever"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let code = decode_list(com, &args, 0)?;
                let local_params = vec![];
                let looping_code = code + "\n__loop";
                int.interpret_in_new_scope(&looping_code, local_params)?;
                Ok(Token::Void)
            },
        )
    }

    pub fn dotimes() -> Self {
        Command::reserved(
            String::from("dotimes"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let loop_config = decode_list(com, &args, 0)?;
                let code = decode_list(com, &args, 1)?;
                let list_items = int.parse_list(&loop_config, true)?;
                if let Some(Token::Word(var_name)) = list_items.get(0) {
                    if let Some(Token::Number(count)) = list_items.get(1) {
                        for index in 0..(*count as usize) {
                            let local_params =
                                vec![(var_name.clone(), Token::Number(index as f32))];
                            int.interpret_in_new_scope(&code, local_params)?;
                        }
                        Ok(Token::Void)
                    } else {
                        Err(Box::from("dotimes expected number for input 1 in input 0"))
                    }
                } else {
                    Err(Box::from("dotimes expected word for input 0 in input 0"))
                }
            },
        )
    }

    pub fn dolist() -> Self {
        Command::reserved(
            String::from("dolist"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let loop_config = decode_list(com, &args, 0)?;
                let code = decode_list(com, &args, 1)?;
                let list_items = int.parse_list(&loop_config, false)?;
                if let Some(Token::Word(var_name)) = list_items.get(0) {
                    if let Some(Token::List(list)) = list_items.get(1) {
                        let list_items = int.parse_list(&list, true)?;
                        for item in list_items {
                            let local_params = vec![(var_name.clone(), item)];
                            int.interpret_in_new_scope(&code, local_params)?;
                        }
                        Ok(Token::Void)
                    } else {
                        Err(Box::from("dolist expected list for input 1 in input 0"))
                    }
                } else {
                    Err(Box::from("dolist expected word for input 0 in input 0"))
                }
            },
        )
    }

    pub fn carefully() -> Self {
        Command::reserved(
            String::from("carefully"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("errormessage"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let error = int.state.data.get_last_error_message();
                Ok(Token::Word(error))
            },
        )
    }

    pub fn paren() -> Self {
        Command::reserved(
            String::from("__paren"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let code = decode_list(com, &args, 0)?;
                let code_with_return = format!("op {}", code);
                int.interpret_in_parenthesis(&code_with_return)
            },
        )
    }
}
