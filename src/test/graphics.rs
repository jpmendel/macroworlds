#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::Interpreter;
    use crate::interpreter::language::token::Token;

    #[test]
    fn turtle_movement() {
        let code = "
        newturtle \"t1
        tto \"t1
        setpos [0 0]
        seth 0
        fd 50
        rt 90
        fd 100
        lt 90
        bk 20
        make \"result pos
        ";
        let mut int = Interpreter::new();
        match int.interpret(&code) {
            Ok(..) => (),
            Err(err) => panic!("test failed: {}", err),
        };
        assert!(
            int.state.data.get_variable("result") == Some(&Token::List(String::from("100 30")))
        );
    }
}
