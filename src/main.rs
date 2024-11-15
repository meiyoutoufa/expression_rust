
use crate::expression::{Expression};

mod expression;
mod stack;

fn main() {
    println!("Hello, world!");
    let mut exp=Expression::new();
    let mut expr = "10 + 5 * ( 20 - 10 / 2 + 1 ) - 100 / 10 > 30 && 30 < 100 || 20 * 3 > 100 || 1 != 100";
    let out = exp.evaluate_expression(expr);
    println!("{:?}", out);
}
