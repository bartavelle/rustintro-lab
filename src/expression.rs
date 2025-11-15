use std::collections::HashMap;

use crate::error::{Expr, Operator};

// Add a new method that works on Expr

fn list_variables(e: &Expr) -> Vec<String> {
    todo!()
}

// Now how to add a new variant to Expr that will let us reuse the code we already have?
//
//
// Really, how?

// let's do something else : open union

pub enum LR<A, B> {
    L(A),
    R(B),
}

trait Eval {
    fn eval(&self, vars: &HashMap<&str, i64>) -> Option<i64>;
}

impl<A: Eval, B: Eval> Eval for LR<A, B> {
    fn eval(&self, vars: &HashMap<&str, i64>) -> Option<i64> {
        todo!()
    }
}

struct SBinop<E> {
    l: Box<E>,
    o: Operator,
    r: Box<E>,
}

struct SVar(String);
struct SLit(i64);

struct NewExpr(LR<SBinop<NewExpr>, LR<SVar, SLit>>);

impl From<SBinop<NewExpr>> for NewExpr {
    fn from(value: SBinop<NewExpr>) -> Self {
        todo!()
    }
}

impl From<&str> for NewExpr {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<i64> for NewExpr {
    fn from(value: i64) -> Self {
        todo!()
    }
}

impl Eval for SVar {
    fn eval(&self, vars: &HashMap<&str, i64>) -> Option<i64> {
        todo!()
    }
}

impl Eval for SLit {
    fn eval(&self, _vars: &HashMap<&str, i64>) -> Option<i64> {
        todo!()
    }
}

impl Eval for NewExpr {
    fn eval(&self, vars: &HashMap<&str, i64>) -> Option<i64> {
        todo!()
    }
}

impl<E: Eval> Eval for SBinop<E> {
    fn eval(&self, vars: &HashMap<&str, i64>) -> Option<i64> {
        todo!()
    }
}

/*
  1/ create a new struct, called Square<E>
  2/ implement the eval trait
  3/ create a new Expr type that integrates the new square operation
  4/ check that is works by writing a test!
*/

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::binop;

    #[test]
    fn t_list_variables() {
        use Operator::*;
        let o = binop(Sub, binop(Add, "a", binop(Sub, "b", "c")), 5);
        assert_eq!(
            list_variables(&o),
            ["a", "b", "c"].into_iter().map(String::from).collect::<Vec<_>>()
        )
    }

    #[test]
    fn open_eval() {
        let vars = HashMap::from([("a", 15_i64)]);
        let expr = NewExpr::from(SBinop {
            l: Box::new("a".into()),
            r: Box::new(45.into()),
            o: Operator::Add,
        });
        assert_eq!(expr.eval(&vars), Some(60))
    }
}
