use rockscript_core::{interpreter::{Interpreter, Value}, lexer::tokenize, parser::Parser};

fn eval(source: &str) -> Value {
    let tokens = tokenize(source, false).unwrap();
    let program = Parser::new(tokens, false).parse().unwrap();
    Interpreter::new().run(program).unwrap()
}

fn eval_err(source: &str) -> String {
    let tokens = tokenize(source, false).unwrap();
    let program = Parser::new(tokens, false).parse().unwrap();
    Interpreter::new().run(program).unwrap_err().desc
}

mod arithmetic {
    use super::*;
    use rockscript_core::interpreter::Value;

    #[test]
    fn add_assign() {
        assert_eq!(eval("throw 4 rocks at x\nsmash 2 into x\nx"), Value::Number(6.0));
    }

    #[test]
    fn sub_assign() {
        assert_eq!(eval("throw 4 rocks at x\nchip 2 off x\nx"), Value::Number(-2.0));
    }

    #[test]
    fn mul_assign() {
        assert_eq!(eval("throw 4 rocks at x\nmate 2 with x\nx"), Value::Number(8.0));
    }

    #[test]
    fn div_assign() {
        assert_eq!(eval("throw 4 rocks at x\nsplit 2 from x\nx"), Value::Number(0.5));
    }

    #[test]
    fn infix_add() {
        assert_eq!(eval("1 smashed into 2"), Value::Number(3.0));
    }

    #[test]
    fn infix_sub() {
        assert_eq!(eval("2 chipped off 1"), Value::Number(-1.0));
    }

    #[test]
    fn infix_mul() {
        assert_eq!(eval("2 mated with 3"), Value::Number(6.0));
    }

    #[test]
    fn infix_div() {
        assert_eq!(eval("6 split from 2"), Value::Number(1.0 / 3.0));
    }

    #[test]
    fn grouped_precedence() {
        assert_eq!(
            eval("(2 mated with 3) smashed into (1 chipped off 4)"),
            Value::Number(9.0)
        );
        assert_eq!(
            eval("1 smashed into (2 mated with 3)"),
            Value::Number(7.0)
        );
        assert_eq!(
            eval("(1 smashed into 2) mated with (3 smashed into 4)"),
            Value::Number(21.0)
        );
    }
}

mod strings {
    use super::*;
    use rockscript_core::interpreter::Value;

    #[test]
    fn string_add_assign() {
        assert_eq!(
            eval(r#"throw rock named "Hello" at s
smash " world" into s
s"#),
            Value::String(" worldHello".to_string())
        );
    }

    #[test]
    fn string_mul_assign_rhs() {
        assert_eq!(
            eval(r#"throw rock named "Hello" at s
mate 3 with s
s"#),
            Value::String("HelloHelloHello".to_string())
        );
    }

    #[test]
    fn number_times_string_lhs() {
        assert_eq!(
            eval(r#""ab" mated with 2"#),
            Value::String("abab".to_string())
        );
    }
}

mod functions {
    use super::*;
    use rockscript_core::interpreter::Value;

    #[test]
    fn call_with_args_and_concat() {
        assert_eq!(
            eval(r#"carve instruction into greet
    retrieve input
    throw rock named "hello " at s
    smash input into s
    engrave s
enough
follow greet with "KirbyTheBurger""#),
            Value::String("KirbyTheBurgerhello ".to_string())
        );
    }

    #[test]
    fn engrave_returns_value() {
        assert_eq!(
            eval(r#"carve instruction into ret
    engrave "returned"
enough
follow ret"#),
            Value::String("returned".to_string())
        );
    }
}

mod comparisons {
    use super::*;
    use rockscript_core::interpreter::Value;

    #[test]
    fn weigh_true_branch() {
        assert_eq!(
            eval(r#"carve instruction into cmp
    retrieve a
    retrieve b
    inspect weigh a against b
        engrave "a >= b"
    refine
        engrave "a < b"
    enough
enough
follow cmp with 5 and 4"#),
            Value::String("a >= b".to_string())
        );
    }

    #[test]
    fn weigh_false_branch() {
        assert_eq!(
            eval(r#"carve instruction into cmp
    retrieve a
    retrieve b
    inspect weigh a against b
        engrave "a >= b"
    refine
        engrave "a < b"
    enough
enough
follow cmp with 4 and 5"#),
            Value::String("a < b".to_string())
        );
    }

    #[test]
    fn else_if_chain() {
        assert_eq!(
            eval(r#"inspect small
refine inspect big
    "correct"
enough"#),
            Value::String("correct".to_string())
        );
    }
}

mod while_loop {
    use super::*;
    use rockscript_core::interpreter::Value;

    #[test]
    fn counts_up_to_condition() {
        assert_eq!(
            eval(r#"throw 0 rocks at x
roll while weigh 2 against x
    smash 1 into x
enough
x"#),
            Value::Number(3.0)
        );
    }
}

mod errors {
    use super::*;

    #[test]
    fn unknown_variable_errors() {
        let desc = eval_err("print(nonexistent)");
        assert!(desc.contains("unknown variable") || desc.to_lowercase().contains("variable"));
    }

    #[test]
    fn mismatched_type_arithmetic_errors() {
        let desc = eval_err("true smashed into false");
        assert!(!desc.is_empty());
    }
}
