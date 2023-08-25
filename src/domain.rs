// Abstract domains
use crate::concrete::{BinOp, Const, Rel, Var};
use AbstractValue::*;

// Non-relational abstraction

#[derive(Debug, PartialEq)]
enum AbstractValue {
    Bottom,
    Top,
    Pos,
    Neg,
}

pub fn includes(a1: AbstractValue, a2: AbstractValue) -> bool {
    a1 == AbstractValue::Bottom || a2 == AbstractValue::Top || a1 == a2
}

pub fn constant(n: Const) -> AbstractValue {
    if n.0 < 0 {
        AbstractValue::Neg
    } else {
        AbstractValue::Pos
    }
}

// Over-approximates the effect of condition tests
// v comp n
pub fn condition(rel: Rel, n: i64, v: AbstractValue) -> AbstractValue {
    if v == AbstractValue::Bottom {
        AbstractValue::Bottom
    } else if rel == Rel::InfEq && n < 0 {
        if v == AbstractValue::Pos {
            AbstractValue::Bottom
        } else {
            AbstractValue::Neg
        }
    } else if rel == Rel::Sup && n >= 0 {
        if v == AbstractValue::Neg {
            AbstractValue::Bottom
        } else {
            AbstractValue::Pos
        }
    } else {
        v
    }
}

pub fn join(a1: AbstractValue, a2: AbstractValue) -> AbstractValue {
    match (a1, a2) {
        (Bottom, a) | (a, Bottom) => a,
        (Top, _) | (_, Top) | (Pos, Neg) | (Neg, Pos) => Top,
        (Pos, Pos) => Pos,
        (Neg, Neg) => Neg,
    }
}

pub fn binop(op: BinOp, a1: AbstractValue, a2: AbstractValue) -> AbstractValue {
    match (op, a1, a2) {
        (_, Bottom, _) | (_, _, Bottom) => Bottom,
        (_, Top, _) | (_, _, Top) => Top,
        (BinOp::Add, Pos, Pos) => Pos,
        (BinOp::Add, Neg, Neg) => Neg,
        (BinOp::Add, _, _) => Top,
        (BinOp::Sub, Neg, Pos) => Neg,
        (BinOp::Sub, Pos, Neg) => Pos,
        (BinOp::Sub, _, _) => Top,
        (BinOp::Mul, Pos, Pos) => Pos,
        (BinOp::Mul, Neg, Neg) => Pos,
        (BinOp::Mul, Neg, Pos) | (BinOp::Mul, Pos, Neg) => Neg,
    }
}

struct AbstractDomain {
    domain: Vec<AbstractValue>,
}

impl AbstractDomain {
    pub fn read(&self, x: Var) -> AbstractValue {
        self.domain[x.0]
    }

    pub fn write(self, x: Var, a: AbstractValue) -> Self {
        self.domain[x] = a;
        self
    }
}
