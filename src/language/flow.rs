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
                let local_params = vec![(String::from("__loopcount"), Token::Number(count as f32))];
                let looping_code = code + "\n__loopback";
                int.interpret_in_new_scope(&looping_code, local_params)?;
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
                let looping_code = code + "\n__loopback";
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
                let setup = decode_list(com, &args, 0)?;
                let code = decode_list(com, &args, 1)?;
                let items = int.parse_list(&setup)?;
                if let Some(Token::Word(var_name)) = items.get(0) {
                    if let Some(Token::Word(count)) = items.get(1) {
                        if let Ok(count) = count.parse::<f32>() {
                            let local_params = vec![
                                (String::from("__loopcount"), Token::Number(count)),
                                (var_name.clone(), Token::Number(0.0)),
                            ];
                            let looping_code =
                                format!("make \"{} difference {} :__loopcount\n", var_name, count)
                                    + &code
                                    + "\n__loopback";
                            int.interpret_in_new_scope(&looping_code, local_params)?;
                            return Ok(Token::Void);
                        }
                    }
                    Err(Box::from("dotimes expected number for input 1 in input 0"))
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
                let setup = decode_list(com, &args, 0)?;
                let code = decode_list(com, &args, 1)?;
                let items = int.parse_list(&setup)?;
                if let Some(Token::Word(var_name)) = items.get(0) {
                    if let Some(Token::List(list)) = items.get(1) {
                        let items = int.parse_list(&list)?;
                        let count = items.len() as f32;
                        let joined = join_to_list_string(items);
                        let local_params = vec![
                            (String::from("__loopcount"), Token::Number(count.clone())),
                            (var_name.clone(), Token::Word(String::new())),
                        ];
                        let looping_code = format!(
                            "make \"{} item {} - :__loopcount [{}]\n",
                            var_name, count, joined
                        ) + &code
                            + "\n__loopback";
                        int.interpret_in_new_scope(&looping_code, local_params)?;
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
                if let Err(..) = int.interpret(&check_code) {
                    int.interpret(&error_code)?;
                }
                Ok(Token::Void)
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

    pub fn loopback() -> Self {
        Command::reserved(
            String::from("__loopback"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let loop_var_name = String::from("__loopcount");
                let count_token = int.state.get_variable(&loop_var_name);
                if let Some(Token::Number(count)) = count_token {
                    let next_count = count - 1.0;
                    if next_count > 0.0 {
                        int.lexer.return_to_start_of_top_frame();
                        int.state
                            .set_variable(loop_var_name, Token::Number(next_count));
                    }
                } else {
                    int.lexer.return_to_start_of_top_frame();
                }
                Ok(Token::Void)
            },
        )
    }
}
