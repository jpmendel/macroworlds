use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params};
use crate::language::token::Token;
use crate::language::util::decode_boolean;
use crate::language::util::{decode_list, decode_number, join_to_list_string};

impl Command {
    pub fn ifthen() -> Self {
        Command {
            name: String::from("if"),
            params: Params::Fixed(2),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let condition = decode_boolean(args.get(0))?;
                let true_code = decode_list(args.get(1))?;
                if condition {
                    int.interpret(&true_code)?;
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn ifelse() -> Self {
        Command {
            name: String::from("ifelse"),
            params: Params::Fixed(3),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let condition = decode_boolean(args.get(0))?;
                let true_code = decode_list(args.get(1))?;
                let false_code = decode_list(args.get(2))?;
                if condition {
                    int.interpret(&true_code)?;
                } else {
                    int.interpret(&false_code)?;
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn repeat() -> Self {
        Command {
            name: String::from("repeat"),
            params: Params::Fixed(2),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let count = decode_number(args.get(0))? as usize;
                let code = decode_list(args.get(1))?;
                let local_params = vec![(String::from("__loopcount"), Token::Number(count as f32))];
                let looping_code = code + "\n__loopback";
                int.interpret_in_new_scope(&looping_code, local_params)?;
                Ok(Token::Void)
            },
        }
    }

    pub fn forever() -> Self {
        Command {
            name: String::from("forever"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let code = decode_list(args.get(0))?;
                let local_params = vec![];
                let looping_code = code + "\n__loopback";
                int.interpret_in_new_scope(&looping_code, local_params)?;
                Ok(Token::Void)
            },
        }
    }

    pub fn dotimes() -> Self {
        Command {
            name: String::from("dotimes"),
            params: Params::Fixed(2),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let setup = decode_list(args.get(0))?;
                let code = decode_list(args.get(1))?;
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
                }
                Err(Box::from("invalid dotimes input"))
            },
        }
    }

    pub fn dolist() -> Self {
        Command {
            name: String::from("dolist"),
            params: Params::Fixed(2),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let setup = decode_list(args.get(0))?;
                let code = decode_list(args.get(1))?;
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
                        return Ok(Token::Void);
                    }
                }
                Err(Box::from("invalid dolist input"))
            },
        }
    }

    pub fn paren() -> Self {
        Command {
            name: String::from("__paren"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let code = decode_list(args.get(0))?;
                let code_with_return = format!("op {}", code);
                int.interpret_in_parenthesis(&code_with_return)
            },
        }
    }

    pub fn loopback() -> Self {
        Command {
            name: String::from("__loopback"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
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
        }
    }
}
