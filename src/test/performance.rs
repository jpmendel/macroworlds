#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::Interpreter;
    use rand::{seq::SliceRandom, Rng};
    use std::collections::HashSet;

    #[test]
    fn math_operators() {
        let num_executions: usize = 20;
        let mut code = String::new();

        for _ in 0..num_executions {
            code += &format!(
                "{} + {}\n",
                rand::thread_rng().gen_range(-1000..=1000),
                rand::thread_rng().gen_range(-1000..=1000)
            );
            code += &format!(
                "(sum {} {} {} {})\n",
                rand::thread_rng().gen_range(-1000..=1000),
                rand::thread_rng().gen_range(-1000..=1000),
                rand::thread_rng().gen_range(-1000..=1000),
                rand::thread_rng().gen_range(-1000..=1000)
            );
            code += &format!(
                "{} - {}\n",
                rand::thread_rng().gen_range(-1000..=1000),
                rand::thread_rng().gen_range(-1000..=1000)
            );
            code += &format!(
                "{} * {}\n",
                rand::thread_rng().gen_range(-500..=500),
                rand::thread_rng().gen_range(-500..=500)
            );
            code += &format!(
                "(product {} {} {} {})\n",
                rand::thread_rng().gen_range(-300..=300),
                rand::thread_rng().gen_range(-300..=300),
                rand::thread_rng().gen_range(-300..=300),
                rand::thread_rng().gen_range(-300..=300)
            );
            code += &format!(
                "{} / {}\n",
                rand::thread_rng().gen_range(-1000..=1000) + 1,
                rand::thread_rng().gen_range(-500..=500) + 1
            );
            code += &format!(
                "{} ^ {}\n",
                rand::thread_rng().gen_range(-1000..=1000),
                rand::thread_rng().gen_range(-10..=10)
            );
            code += &format!(
                "{} % {}\n",
                rand::thread_rng().gen_range(-1000..=1000) + 1,
                rand::thread_rng().gen_range(-500..=500) + 1
            );
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("sum"),
            String::from("difference"),
            String::from("product"),
            String::from("quotient"),
            String::from("power"),
            String::from("remainder"),
        ]));
    }

    #[test]
    fn conditionals() {
        let num_executions: usize = 20;
        let mut code = String::new();
        let booleans = vec!["true", "false"];

        for _ in 0..num_executions {
            code += &format!("if {} = 0 []\n", rand::thread_rng().gen_range(-5..=5));
            code += &format!(
                "ifelse {} = 0 [] []\n",
                rand::thread_rng().gen_range(-5..=5),
            );
            code += &format!("if {} > 0 []\n", rand::thread_rng().gen_range(-5..=5));
            code += &format!("ifelse {} > 0 [] []\n", rand::thread_rng().gen_range(-5..5));
            code += &format!("if {} < 0 []\n", rand::thread_rng().gen_range(-5..=5));
            code += &format!(
                "ifelse {} < 0 [] []\n",
                rand::thread_rng().gen_range(-5..=5),
            );
            code += &format!(
                "if or {} {} []\n",
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!(
                "if (or {} {} {}) []\n",
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!(
                "if and {} {} []\n",
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!(
                "if (and {} {} {})[]\n",
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!(
                "if not {} []\n",
                booleans.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!(
                "if and or not {} {} {} []\n",
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap(),
                booleans.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!("carefully [missing] [show errormessage]\n");
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("if"),
            String::from("ifelse"),
            String::from("carefully"),
            String::from("or"),
            String::from("and"),
            String::from("not"),
            String::from("equal?"),
            String::from("greater?"),
            String::from("less?"),
            String::from("errormessage"),
        ]));
    }

    #[test]
    fn loops() {
        let num_executions: usize = 20;
        let mut code = String::new();
        let num_loops = 10;
        let loop_values = "[1 2 3 4 5 6 7 8 9 10]";

        for _ in 0..num_executions {
            code += &format!("repeat {} [fd 5 rt 30]\n", num_loops);
            code += &format!("dotimes [i {}] [fd :i rt 30]\n", num_loops);
            code += &format!("dolist [i {}] [fd :i rt 30]\n", loop_values);
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("repeat"),
            String::from("dotimes"),
            String::from("dolist"),
        ]));
    }

    #[test]
    fn variables() {
        let num_executions: usize = 20;
        let mut code = String::new();

        for index in 0..num_executions {
            code += &format!("make \"num {}\n", index);
            code += &format!("make \"word \"text{}\n", index);
            code += &format!("make \"list [{} {} {}]\n", index, index + 1, index + 2);
            code += &format!("let [temp1 {} temp2 \"text{}]", index, index,);
            code += &format!("let [temp3 [{} {} {}]]", index, index + 1, index + 2);
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter
            .performance
            .print_summary(HashSet::from([String::from("make"), String::from("let")]));
    }

    #[test]
    fn list_processing() {
        let num_executions: usize = 20;
        let mut code = String::new();

        code += "make \"ls [one two three four]\n";
        for _ in 0..num_executions {
            code += &format!("list \"five \"six \"seven\n");
            code += &format!("fput \"zero :ls\n");
            code += &format!("lput \"five :ls\n");
            code += &format!("first :ls\n");
            code += &format!("last :ls\n");
            code += &format!("butfirst :ls\n");
            code += &format!("butlast :ls\n");
            code += &format!("member? \"two :ls\n");
            code += &format!("empty? :ls\n");
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("list"),
            String::from("fput"),
            String::from("lput"),
            String::from("first"),
            String::from("last"),
            String::from("butfirst"),
            String::from("butlast"),
            String::from("member?"),
            String::from("empty?"),
        ]));
    }

    #[test]
    fn turtle_movement_pen_down() {
        let num_executions: usize = 20;
        let mut code = String::new();

        code += "pd\n";
        for _ in 0..num_executions {
            code += &format!("forward {}\n", rand::thread_rng().gen_range(0..=50));
            code += &format!("right {}\n", rand::thread_rng().gen_range(0..360));
            code += &format!("back {}\n", rand::thread_rng().gen_range(0..=50));
            code += &format!("left {}\n", rand::thread_rng().gen_range(0..360));
            code += &format!("setx {}\n", rand::thread_rng().gen_range(-300..=300));
            code += &format!("sety {}\n", rand::thread_rng().gen_range(-200..=200));
            code += &format!(
                "setpos [{} {}]\n",
                rand::thread_rng().gen_range(-300..=300),
                rand::thread_rng().gen_range(-200..=200)
            );
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("forward"),
            String::from("back"),
            String::from("left"),
            String::from("right"),
            String::from("setx"),
            String::from("sety"),
            String::from("setpos"),
        ]));
    }

    #[test]
    fn turtle_movement_pen_up() {
        let num_executions: usize = 20;
        let mut code = String::new();

        code += "pu\n";
        for _ in 0..num_executions {
            code += &format!("forward {}\n", rand::thread_rng().gen_range(0..=50));
            code += &format!("right {}\n", rand::thread_rng().gen_range(0..360));
            code += &format!("back {}\n", rand::thread_rng().gen_range(0..=50));
            code += &format!("left {}\n", rand::thread_rng().gen_range(0..360));
            code += &format!("setx {}\n", rand::thread_rng().gen_range(-300..=300));
            code += &format!("sety {}\n", rand::thread_rng().gen_range(-200..=200));
            code += &format!(
                "setpos [{} {}]\n",
                rand::thread_rng().gen_range(-300..=300),
                rand::thread_rng().gen_range(-200..=200)
            );
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("forward"),
            String::from("back"),
            String::from("left"),
            String::from("right"),
            String::from("setx"),
            String::from("sety"),
            String::from("setpos"),
        ]));
    }

    #[test]
    fn get_set_attributes() {
        let num_executions: usize = 20;
        let mut code = String::new();
        let shapes = vec!["triangle", "circle", "square"];

        for _ in 0..num_executions {
            code += &format!("xcor\n");
            code += &format!("ycor\n");
            code += &format!("pos\n");
            code += &format!("seth {}\n", rand::thread_rng().gen_range(0..360));
            code += &format!("heading\n");
            code += &format!("setc {}\n", rand::thread_rng().gen_range(0..=255));
            code += &format!("color\n");
            code += &format!("setpensize {}\n", rand::thread_rng().gen_range(1..=10));
            code += &format!("pensize\n");
            code += &format!(
                "setsh \"{}\n",
                shapes.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!("shape\n");
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("xcor"),
            String::from("ycor"),
            String::from("pos"),
            String::from("seth"),
            String::from("heading"),
            String::from("setc"),
            String::from("color"),
            String::from("setpensize"),
            String::from("pensize"),
            String::from("setsh"),
            String::from("shape"),
        ]));
    }

    #[test]
    fn object_custom_attributes() {
        let num_executions: usize = 20;
        let mut code = String::new();

        for index in 0..num_executions {
            code += &format!("turtlesown \"attr{}\n", index);
        }
        for _ in 0..num_executions {
            code += &format!("setattr0 {}\n", rand::thread_rng().gen_range(-10..10));
            code += &format!("attr0\n");
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("turtlesown"),
            String::from("setattr0"),
            String::from("attr0"),
        ]));
    }

    #[test]
    fn object_creation_deletion() {
        let num_executions: usize = 20;
        let mut code = String::new();

        for index in 0..num_executions {
            let id = index + 2;
            code += &format!("newturtle \"t{}\n", id);
            code += &format!("newtext \"text{}\n", id);
            code += &format!("remove \"t{}\n", id);
            code += &format!("remove \"text{}\n", id);
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("newturtle"),
            String::from("newtext"),
            String::from("remove"),
        ]));
    }

    #[test]
    fn background_colors() {
        let num_executions: usize = 20;
        let mut code = String::new();

        for _ in 0..num_executions {
            code += &format!("setbg {}\n", rand::thread_rng().gen_range(0..=255));
            code += &format!("bg\n");
            code += &format!(
                "setpos [{} {}]\n",
                rand::thread_rng().gen_range(-400..=400),
                rand::thread_rng().gen_range(-300..=300)
            );
            code += &format!("colorunder\n");
            code += &format!("clean\n");
        }

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&code);
        assert!(result.is_ok());

        interpreter.performance.print_summary(HashSet::from([
            String::from("setbg"),
            String::from("bg"),
            String::from("colorunder"),
            String::from("clean"),
        ]));
    }
}
