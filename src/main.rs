use crate::abstract_int::*;
use crate::concrete::*;
use crate::domain::*;
use Command::*;

pub mod abstract_int;
pub mod concrete;
pub mod domain;

fn main() {
    let x = Var::new();

    let program = Command::make_seq(Assign(x, Expr::new_const(3)), Assign(x, Expr::new_const(4)));

    let p2 = seq!(
        Command::assign_const(x, 5),
        Assign(
            x,
            Expr::BinOp {
                op: BinOp::Add,
                left: Box::new(Expr::Var(x)),
                right: Box::new(Expr::new_const(3))
            }
        )
    );

    let mem = Memory::new();

    let mem = mem.sem_com(&(Label::new(), p2));

    println!("Result is {:?}", mem.read(Var(0)))
}
