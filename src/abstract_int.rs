use crate::concrete::*;
use crate::domain::binop;
use crate::domain::*;

pub fn expr(e: &Expr, domain: &AbstractDomain) -> AbstractValue {
    match e {
        Expr::Const(n) => constant(*n),
        Expr::Var(x) => domain.read(*x),
        Expr::BinOp { op, left, right } => binop(*op, expr(left, domain), expr(right, domain)),
    }
}

pub fn ai_cond(cond: &Cond, domain: &AbstractDomain) -> AbstractDomain {
    let approx_cond = condition(cond.rel, cond.right.0, domain.read(cond.left));
    if approx_cond == AbstractValue::Bottom {
        domain.clone().bottomize()
    } else {
        domain.clone().write(cond.left, approx_cond)
    }
}

// Compositional semantics

// For loops
pub fn post_loop<F>(f: F, domain: &AbstractDomain) -> AbstractDomain
where
    F: for<'a> Fn(&'a AbstractDomain) -> AbstractDomain,
{
    let next_domain = f(&domain);
    if next_domain.is_le(&domain) {
        domain.clone()
    } else {
        post_loop(f, &domain.join(&next_domain))
    }
}

use crate::concrete::Command::*;

pub fn command(state: &(Label, Command), domain: AbstractDomain) -> AbstractDomain {
    if domain.is_bottom() {
        domain
    } else {
        match state.1 {
            Skip => domain,
            Seq(ref c1, ref c2) => command(c1, command(c2, domain)),
            Assign(var, ref e) => {
                let a = expr(e, &domain);
                domain.write(var, a)
            }
            Input(var) => domain.write(var, AbstractValue::Top),
            If {
                ref cond,
                ref then,
                ref els,
            } => {
                let cond_true = ai_cond(&cond, &domain);
                let d_true = command(&then, cond_true);
                if els.is_some() {
                    let cond_false = ai_cond(&cond.negate(), &domain);
                    let d_false = command(els.as_ref().unwrap(), cond_false);
                    d_true.join(&d_false)
                } else {
                    d_true
                }
            }
            While { ref cond, ref body } => {
                let f_loop: Box<dyn for<'a> Fn(&'a AbstractDomain) -> AbstractDomain> =
                    Box::new(|a| {
                        let loop_cond = ai_cond(cond, a);
                        command(body, loop_cond)
                    });
                ai_cond(&cond.negate(), &post_loop(f_loop, &domain))
            }
        }
    }
}
