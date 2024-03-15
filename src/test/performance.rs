#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::Interpreter;
    use rand::{seq::SliceRandom, Rng};
    use std::collections::HashSet;

    #[test]
    fn math_operators() {
        assert!(cfg!(feature = "performance"));

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
                rand::thread_rng().gen_range(-200..=200),
                rand::thread_rng().gen_range(-200..=200)
            );
            code += &format!(
                "(product {} {} {} {})\n",
                rand::thread_rng().gen_range(-200..=200),
                rand::thread_rng().gen_range(-200..=200),
                rand::thread_rng().gen_range(-200..=200),
                rand::thread_rng().gen_range(-200..=200)
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

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance.print_summary(HashSet::from([
            "sum",
            "difference",
            "product",
            "quotient",
            "power",
            "remainder",
        ]));

        assert!(int.performance.average("sum").as_nanos() < 2000);
        assert!(int.performance.average("difference").as_nanos() < 2000);
        assert!(int.performance.average("product").as_nanos() < 3000);
        assert!(int.performance.average("quotient").as_nanos() < 3000);
        assert!(int.performance.average("power").as_nanos() < 2000);
        assert!(int.performance.average("remainder").as_nanos() < 2000);
    }

    #[test]
    fn conditionals() {
        assert!(cfg!(feature = "performance"));

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

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance.print_summary(HashSet::from([
            "if",
            "ifelse",
            "carefully",
            "or",
            "and",
            "not",
            "equal?",
            "greater?",
            "less?",
            "errormessage",
        ]));
    }

    #[test]
    fn loops() {
        assert!(cfg!(feature = "performance"));

        let num_executions: usize = 20;
        let mut code = String::new();
        let num_loops = 10;
        let loop_values = "[1 2 3 4 5 6 7 8 9 10]";

        for _ in 0..num_executions {
            code += &format!("repeat {} [fd 5 rt 30]\n", num_loops);
            code += &format!("dotimes [i {}] [fd :i rt 30]\n", num_loops);
            code += &format!("dolist [i {}] [fd :i rt 30]\n", loop_values);
        }

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance
            .print_summary(HashSet::from(["repeat", "dotimes", "dolist"]));
    }

    #[test]
    fn variables() {
        assert!(cfg!(feature = "performance"));

        let num_executions: usize = 20;
        let mut code = String::new();

        for index in 0..num_executions {
            code += &format!("make \"num {}\n", index);
            code += &format!("make \"word \"text{}\n", index);
            code += &format!("make \"list [{} {} {}]\n", index, index + 1, index + 2);
            code += &format!("make \"|spaced word| \"|words in space{}|\n", index);
            code += &format!(
                "make \"|spaced list| [|word space{}| |other space{}|]\n",
                index,
                index + 1
            );
            code += &format!("let [temp1 {} temp2 \"text{}]\n", index, index,);
            code += &format!("let [temp3 [{} {} {}]]\n", index, index + 1, index + 2);
        }

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance
            .print_summary(HashSet::from(["make", "let"]));
    }

    #[test]
    fn list_processing() {
        assert!(cfg!(feature = "performance"));

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

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance.print_summary(HashSet::from([
            "list", "fput", "lput", "first", "last", "butfirst", "butlast", "member?", "empty?",
        ]));
    }

    #[test]
    fn turtle_movement_pen_down() {
        assert!(cfg!(feature = "performance"));

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

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance.print_summary(HashSet::from([
            "forward", "back", "left", "right", "setx", "sety", "setpos",
        ]));
    }

    #[test]
    fn turtle_movement_pen_up() {
        assert!(cfg!(feature = "performance"));

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

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance.print_summary(HashSet::from([
            "forward", "back", "left", "right", "setx", "sety", "setpos",
        ]));
    }

    #[test]
    fn get_set_attributes() {
        assert!(cfg!(feature = "performance"));

        let num_executions: usize = 20;
        let mut code = String::new();
        let shapes = vec!["triangle", "circle", "square"];

        for _ in 0..num_executions {
            code += &format!("xcor\n");
            code += &format!("ycor\n");
            code += &format!("pos\n");
            code += &format!("setheading {}\n", rand::thread_rng().gen_range(0..360));
            code += &format!("heading\n");
            code += &format!("setcolor {}\n", rand::thread_rng().gen_range(0..=255));
            code += &format!("color\n");
            code += &format!("setsize {}\n", rand::thread_rng().gen_range(4..=80));
            code += &format!("size\n");
            code += &format!("setpensize {}\n", rand::thread_rng().gen_range(1..=10));
            code += &format!("pensize\n");
            code += &format!(
                "setsh \"{}\n",
                shapes.choose(&mut rand::thread_rng()).unwrap()
            );
            code += &format!("shape\n");
        }

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance.print_summary(HashSet::from([
            "xcor",
            "ycor",
            "pos",
            "setheading",
            "heading",
            "setcolor",
            "color",
            "setsize",
            "size",
            "setpensize",
            "pensize",
            "setsh",
            "shape",
        ]));
    }

    #[test]
    fn object_custom_attributes() {
        assert!(cfg!(feature = "performance"));

        let num_executions: usize = 20;
        let mut code = String::new();

        for index in 0..num_executions {
            code += &format!("turtlesown \"attr{}\n", index);
        }
        for _ in 0..num_executions {
            code += &format!("setattr0 {}\n", rand::thread_rng().gen_range(-10..10));
            code += &format!("attr0\n");
        }

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance
            .print_summary(HashSet::from(["turtlesown", "setattr0", "attr0"]));
    }

    #[test]
    fn object_creation_deletion() {
        assert!(cfg!(feature = "performance"));

        let num_executions: usize = 20;
        let mut code = String::new();

        for index in 0..num_executions {
            let id = index + 2;
            code += &format!("newturtle \"t{}\n", id);
            code += &format!("newtext \"text{}\n", id);
            code += &format!("remove \"t{}\n", id);
            code += &format!("remove \"text{}\n", id);
        }

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance
            .print_summary(HashSet::from(["newturtle", "newtext", "remove"]));
    }

    #[test]
    fn background_colors() {
        assert!(cfg!(feature = "performance"));

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

        let mut int = Interpreter::new();
        let result = int.interpret(&code);
        assert!(result.is_ok());

        int.performance
            .print_summary(HashSet::from(["setbg", "bg", "colorunder", "clean"]));
    }
}
