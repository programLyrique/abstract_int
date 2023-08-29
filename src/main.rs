use crate::abstract_int::*;
use crate::concrete::*;
use crate::domain::*;
use Command::*;

pub mod abstract_int;
pub mod concrete;
pub mod domain;

fn main() {
    let x = Var::new();

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
    let p3 = p2.clone();

    let mem = Memory::new();

    let mem = mem.sem_com(&(Label::new(), p2));

    println!("Result is {:?}", mem.read(x));

    // And now let's interpret it abstractly!

    let domain = AbstractDomain::new();
    let domain = command(&(Label::new(), p3), domain);
    println!("Abstract result is {:?}", domain.read(x));
}
