#[cfg(test)]
mod tests {
    use crate::interpreter::language::lexer::Lexer;
    use crate::interpreter::language::structure::Command;
    use crate::interpreter::language::token::Token;

    fn read_code_into_tokens(code: &str) -> Vec<Token> {
        let mut lexer = Lexer::new();
        lexer.push_block(code, false);

        let mut tokens: Vec<Token> = vec![];
        while let Ok(token) = lexer.read_token() {
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn read_code_single_line() {
        let code = "forward 50 right 90 back 50 left 90 setpos [20 -30] forward -50 setheading 180";
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(Command::forward(), vec![Token::Number(50.0)]),
            Token::Command(Command::right(), vec![Token::Number(90.0)]),
            Token::Command(Command::back(), vec![Token::Number(50.0)]),
            Token::Command(Command::left(), vec![Token::Number(90.0)]),
            Token::Command(Command::setpos(), vec![Token::List(String::from("20 -30"))]),
            Token::Command(Command::forward(), vec![Token::Number(-50.0)]),
            Token::Command(Command::setheading(), vec![Token::Number(180.0)]),
        ];

        assert_eq!(tokens, expect);
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
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(Command::setpos(), vec![Token::List(String::from("-15 45"))]),
            Token::Command(Command::back(), vec![Token::Number(30.0)]),
            Token::Command(Command::forward(), vec![Token::Number(50.0)]),
            Token::Command(Command::left(), vec![Token::Number(90.0)]),
            Token::Command(Command::setheading(), vec![Token::Number(0.0)]),
            Token::Command(Command::back(), vec![Token::Number(-50.0)]),
            Token::Command(Command::right(), vec![Token::Number(-90.0)]),
        ];

        assert_eq!(tokens, expect);
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
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(
                Command::make(),
                vec![Token::Word(String::from("var")), Token::Number(50.0)],
            ),
            Token::Command(
                Command::forward(),
                vec![Token::Variable(String::from("var"))],
            ),
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var")),
                    Token::Command(
                        Command::difference(),
                        vec![Token::Variable(String::from("var")), Token::Number(5.0)],
                    ),
                ],
            ),
            Token::Command(Command::back(), vec![Token::Variable(String::from("var"))]),
            Token::Command(
                Command::ifthen(),
                vec![
                    Token::Command(
                        Command::equal(),
                        vec![Token::Variable(String::from("var")), Token::Number(45.0)],
                    ),
                    Token::List(String::from("bk 20")),
                ],
            ),
            Token::Command(
                Command::repeat(),
                vec![Token::Number(3.0), Token::List(String::from("fd :var"))],
            ),
        ];

        assert_eq!(tokens, expect);
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
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(
                Command::to(),
                vec![Token::Procedure(
                    String::from("function1"),
                    vec![],
                    String::from("code"),
                )],
            ),
            Token::Command(
                Command::to(),
                vec![Token::Procedure(
                    String::from("function2"),
                    vec![String::from("param")],
                    String::from("code"),
                )],
            ),
        ];

        assert_eq!(tokens, expect);
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
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("lst")),
                    Token::List(String::from("one two three")),
                ],
            ),
            Token::Command(
                Command::make(),
                vec![Token::Word(String::from("var")), Token::Number(10.0)],
            ),
            Token::Command(
                Command::list(),
                vec![
                    Token::Variable(String::from("lst")),
                    Token::List(String::from("four five")),
                ],
            ),
            Token::Command(
                Command::dotimes(),
                vec![
                    Token::List(String::from("i 5")),
                    Token::List(String::from("show :i")),
                ],
            ),
            Token::Command(
                Command::dolist(),
                vec![
                    Token::List(String::from("i [one two :var]")),
                    Token::List(String::from("show :i")),
                ],
            ),
        ];

        assert_eq!(tokens, expect);
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
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var1")),
                    Token::Word(String::from("hello")),
                ],
            ),
            Token::Command(
                Command::make(),
                vec![Token::Word(String::from("var2")), Token::Number(10.0)],
            ),
            Token::Command(
                Command::ifthen(),
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
            Token::Command(
                Command::ifthen(),
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
                                vec![Token::Variable(String::from("var2")), Token::Number(5.0)],
                            ),
                        ],
                    ),
                    Token::List(String::from("show \"yes")),
                ],
            ),
            Token::Command(
                Command::ifelse(),
                vec![
                    Token::Command(
                        Command::not(),
                        vec![Token::Command(
                            Command::less(),
                            vec![Token::Variable(String::from("var2")), Token::Number(5.0)],
                        )],
                    ),
                    Token::List(String::from("show \"yes")),
                    Token::List(String::from("show \"no")),
                ],
            ),
            Token::Command(
                Command::carefully(),
                vec![
                    Token::List(String::from("show :noexist")),
                    Token::List(String::from("show \"no")),
                ],
            ),
        ];

        assert_eq!(tokens, expect);
    }

    #[test]
    fn read_code_with_infix_operators() {
        let code = "
        make \"var1 6 + 5
        make \"var2 10 - 4
        make \"var3 3 * 7
        make \"var4 8 / 2
        make \"var5 4 ^ 2
        make \"var6 10 % 3
        ";
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var1")),
                    Token::Command(Command::sum(), vec![Token::Number(6.0), Token::Number(5.0)]),
                ],
            ),
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var2")),
                    Token::Command(
                        Command::difference(),
                        vec![Token::Number(10.0), Token::Number(4.0)],
                    ),
                ],
            ),
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var3")),
                    Token::Command(
                        Command::product(),
                        vec![Token::Number(3.0), Token::Number(7.0)],
                    ),
                ],
            ),
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var4")),
                    Token::Command(
                        Command::quotient(),
                        vec![Token::Number(8.0), Token::Number(2.0)],
                    ),
                ],
            ),
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var5")),
                    Token::Command(
                        Command::power(),
                        vec![Token::Number(4.0), Token::Number(2.0)],
                    ),
                ],
            ),
            Token::Command(
                Command::make(),
                vec![
                    Token::Word(String::from("var6")),
                    Token::Command(
                        Command::remainder(),
                        vec![Token::Number(10.0), Token::Number(3.0)],
                    ),
                ],
            ),
        ];

        assert_eq!(tokens, expect);
    }

    #[test]
    fn read_code_with_parenthesis() {
        let code = "
        show (5 + 2) * (3 - 6)
        setpos list ((minus 4) + 3) (exp 6 - 1)
        ";
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(
                Command::show(),
                vec![Token::Command(
                    Command::product(),
                    vec![
                        Token::Command(Command::paren(), vec![Token::List(String::from("5 + 2"))]),
                        Token::Command(Command::paren(), vec![Token::List(String::from("3 - 6"))]),
                    ],
                )],
            ),
            Token::Command(
                Command::setpos(),
                vec![Token::Command(
                    Command::list(),
                    vec![
                        Token::Command(
                            Command::paren(),
                            vec![Token::List(String::from("(minus 4) + 3"))],
                        ),
                        Token::Command(
                            Command::paren(),
                            vec![Token::List(String::from("exp 6 - 1"))],
                        ),
                    ],
                )],
            ),
        ];

        assert_eq!(tokens, expect);
    }

    #[test]
    fn read_code_with_aliases() {
        let code = "
        fd 40
        bk 20
        rt 90
        lt 45
        se \"one \"two
        sentence \"one \"two
        op \"result
        tto :turtle
        ";
        let tokens = read_code_into_tokens(code);

        let expect = vec![
            Token::Command(Command::forward(), vec![Token::Number(40.0)]),
            Token::Command(Command::back(), vec![Token::Number(20.0)]),
            Token::Command(Command::right(), vec![Token::Number(90.0)]),
            Token::Command(Command::left(), vec![Token::Number(45.0)]),
            Token::Command(
                Command::list(),
                vec![
                    Token::Word(String::from("one")),
                    Token::Word(String::from("two")),
                ],
            ),
            Token::Command(
                Command::list(),
                vec![
                    Token::Word(String::from("one")),
                    Token::Word(String::from("two")),
                ],
            ),
            Token::Command(Command::output(), vec![Token::Word(String::from("result"))]),
            Token::Command(
                Command::talkto(),
                vec![Token::Variable(String::from("turtle"))],
            ),
        ];

        assert_eq!(tokens, expect);
    }
}
