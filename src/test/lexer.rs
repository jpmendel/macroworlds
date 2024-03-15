#[cfg(test)]
mod tests {
    use crate::interpreter::language::command::command::Command;
    use crate::interpreter::language::token::Token;
    use crate::interpreter::lexer::Lexer;

    fn check_expected_tokens(tokens: Vec<Token>, expect: Vec<(&str, Vec<Token>)>) {
        for index in 0..expect.len() {
            let (expected_name, expected_args) = &expect[index];
            let Token::Command(command, args) = &tokens[index] else {
                panic!("expected command: {}", expected_name);
            };
            assert!(command.is(expected_name));
            for index in 0..expected_args.len() {
                assert_eq!(args[index], expected_args[index]);
            }
        }
    }

    #[test]
    fn read_code_single_line() {
        let code = "forward 50 right 90 back 50 left 90 setpos [20 -30] forward -50 setheading 180";
        let mut lexer = Lexer::new();
        lexer.push_block(code, false);

        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = lexer.read_token() {
            tokens.push(token);
        }

        let expect = vec![
            ("forward", vec![Token::Number(50.0)]),
            ("right", vec![Token::Number(90.0)]),
            ("back", vec![Token::Number(50.0)]),
            ("left", vec![Token::Number(90.0)]),
            ("setpos", vec![Token::List(String::from("20 -30"))]),
            ("forward", vec![Token::Number(-50.0)]),
            ("setheading", vec![Token::Number(180.0)]),
        ];

        check_expected_tokens(tokens, expect);
    }

    #[test]
    fn read_code_multi_line() {
        let code = "
        setpos [-15 45]
        back 30
        forward 50
        left 90
        setheading 0
        back -50
        right -90
        ";
        let mut lexer = Lexer::new();
        lexer.push_block(code, false);

        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = lexer.read_token() {
            tokens.push(token);
        }

        let expect = vec![
            ("setpos", vec![Token::List(String::from("-15 45"))]),
            ("back", vec![Token::Number(30.0)]),
            ("forward", vec![Token::Number(50.0)]),
            ("left", vec![Token::Number(90.0)]),
            ("setheading", vec![Token::Number(0.0)]),
            ("back", vec![Token::Number(-50.0)]),
            ("right", vec![Token::Number(-90.0)]),
        ];

        check_expected_tokens(tokens, expect);
    }

    #[test]
    fn read_code_with_variables() {
        let code = "
        make \"var 50
        forward :var
        make \"var :var - 5
        back :var
        if :var = 45 [bk 20]
        repeat 3 [fd :var]
        ";
        let mut lexer = Lexer::new();
        lexer.push_block(code, false);

        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = lexer.read_token() {
            tokens.push(token);
        }

        let expect = vec![
            (
                "make",
                vec![Token::Word(String::from("var")), Token::Number(50.0)],
            ),
            ("forward", vec![Token::Variable(String::from("var"))]),
            (
                "make",
                vec![
                    Token::Word(String::from("var")),
                    Token::Command(
                        Command::difference(),
                        vec![Token::Variable(String::from("var")), Token::Number(5.0)],
                    ),
                ],
            ),
            ("back", vec![Token::Variable(String::from("var"))]),
            (
                "if",
                vec![Token::Command(
                    Command::equal(),
                    vec![Token::Variable(String::from("var")), Token::Number(45.0)],
                )],
            ),
            (
                "repeat",
                vec![Token::Number(3.0), Token::List(String::from("fd :var"))],
            ),
        ];

        check_expected_tokens(tokens, expect);
    }

    #[test]
    fn read_code_with_procedures() {
        let code = "
        to function1
        fd 50
        rt 90
        end

        to function2 :param
        make \"val :param + 10
        output :val
        end
        ";
        let mut lexer = Lexer::new();
        lexer.push_block(code, false);

        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = lexer.read_token() {
            tokens.push(token);
        }

        let expect = vec![
            (
                "to",
                vec![Token::Procedure(
                    String::from("function1"),
                    vec![],
                    String::from("code"),
                )],
            ),
            (
                "to",
                vec![Token::Procedure(
                    String::from("function2"),
                    vec![String::from("param")],
                    String::from("code"),
                )],
            ),
        ];

        check_expected_tokens(tokens, expect);
    }

    #[test]
    fn read_code_with_list_processing() {
        let code = "
        make \"lst [one two three]
        make \"var 10
        list :lst [four five]
        dotimes [i 5] [show :i]
        dolist [i [one two :var]] [show :i]
        ";
        let mut lexer = Lexer::new();
        lexer.push_block(code, false);

        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = lexer.read_token() {
            tokens.push(token);
        }

        let expect = vec![
            (
                "make",
                vec![
                    Token::Word(String::from("lst")),
                    Token::List(String::from("one two three")),
                ],
            ),
            (
                "make",
                vec![Token::Word(String::from("var")), Token::Number(10.0)],
            ),
            (
                "list",
                vec![
                    Token::Variable(String::from("lst")),
                    Token::List(String::from("four five")),
                ],
            ),
            (
                "dotimes",
                vec![
                    Token::List(String::from("i 5")),
                    Token::List(String::from("show :i")),
                ],
            ),
            (
                "dolist",
                vec![
                    Token::List(String::from("i [one two :var]")),
                    Token::List(String::from("show :i")),
                ],
            ),
        ];

        check_expected_tokens(tokens, expect);
    }

    #[test]
    fn read_code_with_conditionals() {
        let code = "
        make \"var1 \"hello
        make \"var2 10
        if equal? :var1 \"hello [show \"yes]
        if and equal? :var1 \"hello greater? :var2 5 [show \"yes]
        ifelse not less? :var2 5 [show \"yes] [show \"no]
        carefully [show :noexist] [show \"no]
        ";
        let mut lexer = Lexer::new();
        lexer.push_block(code, false);

        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = lexer.read_token() {
            tokens.push(token);
        }

        let expect = vec![
            (
                "make",
                vec![
                    Token::Word(String::from("var1")),
                    Token::Word(String::from("hello")),
                ],
            ),
            (
                "make",
                vec![Token::Word(String::from("var2")), Token::Number(10.0)],
            ),
            (
                "if",
                vec![
                    Token::Command(
                        Command::equal(),
                        vec![
                            Token::Variable(String::from("var1")),
                            Token::Word(String::from("hello")),
                        ],
                    ),
                    Token::List(String::from("show \"yes")),
                ],
            ),
            (
                "if",
                vec![
                    Token::Command(
                        Command::and(),
                        vec![
                            Token::Command(
                                Command::equal(),
                                vec![
                                    Token::Variable(String::from("var1")),
                                    Token::Word(String::from("hello")),
                                ],
                            ),
                            Token::Command(
                                Command::greater(),
                                vec![Token::Variable(String::from("var1")), Token::Number(5.0)],
                            ),
                        ],
                    ),
                    Token::List(String::from("show \"yes")),
                ],
            ),
            (
                "ifelse",
                vec![
                    Token::Command(
                        Command::not(),
                        vec![Token::Command(
                            Command::less(),
                            vec![Token::Variable(String::from("var2")), Token::Number(5.0)],
                        )],
                    ),
                    Token::List(String::from("show \"yes")),
                ],
            ),
            (
                "carefully",
                vec![
                    Token::List(String::from("show :noexist")),
                    Token::List(String::from("show \"no")),
                ],
            ),
        ];

        check_expected_tokens(tokens, expect);
    }

    #[test]
    fn read_code_with_infix_operators() {}

    #[test]
    fn read_code_with_parenthesis() {}

    #[test]
    fn read_code_with_aliases() {}
}
