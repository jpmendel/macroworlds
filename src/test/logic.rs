#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::Interpreter;
    use crate::interpreter::language::token::Token;

    #[test]
    fn math_operators() {
        let code = "
        make \"var1 15
        make \"var2 3
        make \"result1 :var1 + :var2
        make \"result2 :var1 - :var2
        make \"result3 :var1 * :var2
        make \"result4 :var1 / :var2
        make \"result5 :var1 ^ :var2
        make \"result6 :var1 % :var2
        ";
        let mut int = Interpreter::new();
        match int.interpret(&code) {
            Ok(..) => (),
            Err(err) => panic!("test failed: {}", err),
        };
        assert!(int.state.data.get_variable("result1") == Some(&Token::Number(18.0)));
        assert!(int.state.data.get_variable("result2") == Some(&Token::Number(12.0)));
        assert!(int.state.data.get_variable("result3") == Some(&Token::Number(45.0)));
        assert!(int.state.data.get_variable("result4") == Some(&Token::Number(5.0)));
        assert!(int.state.data.get_variable("result5") == Some(&Token::Number(3375.0)));
        assert!(int.state.data.get_variable("result6") == Some(&Token::Number(0.0)));
    }

    #[test]
    fn list_processing() {
        let code = "
        make \"lst [one two three four]
        make \"result1 lput \"five :lst
        make \"result2 butfirst :lst
        make \"result3 item 2 :lst
        ";
        let mut int = Interpreter::new();
        match int.interpret(&code) {
            Ok(..) => (),
            Err(err) => panic!("test failed: {}", err),
        };
        assert!(
            int.state.data.get_variable("result1")
                == Some(&Token::List(String::from("one two three four five")))
        );
        assert!(
            int.state.data.get_variable("result2")
                == Some(&Token::List(String::from("two three four")))
        );
        assert!(
            int.state.data.get_variable("result3") == Some(&Token::Word(String::from("three")))
        );
    }
}
