mod withenum {
    enum Expr {
        Literal(i32),
        Add(Box<Expr>, Box<Expr>),
    }

    impl Expr {
        fn add(self, r: Self) -> Self {
            Expr::Add(Box::new(self), Box::new(r))
        }

        fn lit(v: i32) -> Self {
            Self::Literal(v)
        }

        // fn mul(self, r: Self) -> Self {
        //     Expr::Mul(Box::new(self), Box::new(r))
        // }
    }

    fn eval(expr: &Expr) -> i32 {
        match expr {
            Expr::Literal(n) => *n,
            Expr::Add(l, r) => eval(l) + eval(r),
        }
    }

    // 1. create a print function that looks like :
    // fn print(expr: &Expr) -> String { ... }
    // uncomment the `tprint` test

    // 2. add a new variant, Mul(Box<Expr>, Box<Expr>), and update the code
    // - update all code that needs to me updates
    // - uncomment the `mul` function and the `tmul` test

    /*
    What code did you have to modify to add a new variant?
    What code did you have to modify to add a new operation?
    Which extension was easier? Why?
    */

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn basic() {
            let e = Expr::lit(5).add(Expr::lit(8));
            assert_eq!(eval(&e), 13)
        }

        // #[test]
        // fn tprint() {
        //     let e = Expr::lit(5).add(Expr::lit(8));
        //     assert_eq!(print(&e), "(5 + 8)")
        // }
        //
        // #[test]
        // fn tmul() {
        //     let e = Expr::lit(5).add(Expr::lit(8)).mul(Expr::lit(4));
        //     assert_eq!(eval(&e), 52);
        //     assert_eq!(print(&e), "((5 + 8) * 4)")
        // }
    }
}

mod withtrait {
    trait Eval {
        fn eval(&self) -> i32;
    }

    type Expr = Box<dyn Eval>;

    #[derive(Clone)]
    struct Literal {
        value: i32,
    }

    struct Add {
        left: Expr,
        right: Expr,
    }

    impl Eval for Add {
        fn eval(&self) -> i32 {
            self.left.eval() + self.right.eval()
        }
    }

    impl Eval for Literal {
        fn eval(&self) -> i32 {
            self.value
        }
    }

    // 1. add a new variant, Mul { left: Expr, right: Expr }, and update the code
    // - uncomment the tmul test

    // 2. add a print trait
    // - what happened?

    trait Print {
        fn print(&self) -> String;
    }

    // - can you reuse the previous structures?

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn basic() {
            let e = Add {
                left: Box::new(Literal { value: 5 }),
                right: Box::new(Literal { value: 8 }),
            };
            assert_eq!(e.eval(), 13)
        }

        // #[test]
        // fn tmul() {
        //     let a = Add {
        //         left: Box::new(Literal { value: 5 }),
        //         right: Box::new(Literal { value: 8 }),
        //     };
        //     let e = Mul {
        //         left: Box::new(a),
        //         right: Box::new(Literal { value: 4 }),
        //     };
        //     assert_eq!(e.eval(), 52)
        // }
    }
}
