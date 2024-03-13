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
        let code = "forward 50 right 90 back 50 left 90 setpos [20 -30] forward -50 seth 180";
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
            ("seth", vec![Token::Number(180.0)]),
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
        seth 0
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
            ("seth", vec![Token::Number(0.0)]),
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
}
