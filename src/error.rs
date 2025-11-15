use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Binop(Operator, Box<Expr>, Box<Expr>),
    Int(i64),
    Var(String),
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::Int(value)
    }
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        Expr::Var(value.to_string())
    }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EvalError {
    DivisionByZero,
    UnknownVariable(String),
}

// exercise starts here
fn divide(a: i64, b: i64) -> Result<i64, ()> {
    todo!()
}

fn eval_binop(o: &Operator, a: i64, b: i64) -> Result<i64, EvalError> {
    match o {
        _ => todo!(),
    }
}

pub fn eval(vars: &HashMap<String, i64>, expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        _ => todo!(),
    }
}

pub(crate) fn binop<A: Into<Expr>, B: Into<Expr>>(o: Operator, a: A, b: B) -> Expr {
    Expr::Binop(o, Box::new(a.into()), Box::new(b.into()))
}

#[cfg(test)]
mod test {
    use super::EvalError::*;
    use super::Expr::*;
    use super::Operator::*;
    use super::*;

    fn vars() -> HashMap<String, i64> {
        HashMap::from([("a".to_string(), 1), ("b".to_string(), 2)])
    }

    #[test]
    fn divide_ok() {
        assert_eq!(divide(6, 3), Ok(2))
    }

    #[test]
    fn divide_bad() {
        assert_eq!(divide(6, 0), Err(()))
    }

    #[test]
    fn binop_ok() {
        assert_eq!(eval_binop(&Div, 6, 3), Ok(2))
    }

    #[test]
    fn binop_bad() {
        assert_eq!(eval_binop(&Div, 6, 0), Err(DivisionByZero))
    }

    #[test]
    fn eval_simple() {
        let v = vars();
        assert_eq!(eval(&v, &binop(Add, 12, 3)), Ok(15))
    }

    #[test]
    fn eval_div_by_zero() {
        let v = vars();
        assert_eq!(
            eval(&v, &binop(Div, Int(12), binop(Sub, Int(3), binop(Add, Int(1), Int(2))))),
            Err(DivisionByZero)
        )
    }

    #[test]
    fn eval_variables() {
        let v = vars();
        assert_eq!(eval(&v, &binop(Add, "a", "b")), Ok(3))
    }

    #[test]
    fn eval_unk_variable() {
        let v = vars();
        assert_eq!(
            eval(&v, &binop(Add, "a", binop(Sub, "b", "c"))),
            Err(UnknownVariable("c".to_string()))
        )
    }
}
