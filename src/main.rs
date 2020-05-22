mod evaluator;
mod lexer;
mod parser;
use parser::generate;

fn main() {
    let parser = generate("(¬(p → q) → (p → ¬q))").unwrap();
    println!("{:?}", evaluator::eval(&parser));

    let parser = generate("((p → ¬q) → ¬(p → q))").unwrap();
    println!("{:?}", evaluator::eval(&parser));

    let parser = generate("((p → (q ∧ r)) → ((p → q) ∨ (q → r)))").unwrap();
    println!("{:?}", evaluator::eval(&parser));

    let parser = generate("(((p → q) ∨ (q → r)) → (p → (q ∨ r)))").unwrap();
    println!("{:?}", evaluator::eval(&parser));

    let parser = generate("(p ∨ (p → (q ∧ ¬q)))").unwrap();
    println!("{:?}", evaluator::eval(&parser));
}
