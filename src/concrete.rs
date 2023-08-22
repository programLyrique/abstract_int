// Concrete semantics
struct Var(usize);
struct Const(i64);

enum BinOp {
    Add,
    Sub,
    Mul,
}

enum Rel {
    InfEq,
    Sup,
}

enum Expr {
    Const(Const),
    Var(Var),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

struct Cond {
    rel: Rel,
    left: Var,
    right: Const,
}

// TODO: we need to add the labels, but it is not as easy as in Ocaml where one
// can define mutually recursive types.
enum Command {
    Skip,
    Seq(Box<Command>, Box<Command>),
    Assign(Var, Expr),
    Input(Var),
    If {
        cond: Cond,
        then: Box<Command>,
        els: Option<Command>,
    },
    While {
        cond: Cond,
        body: Box<Command>,
    },
}

struct Memory {
    mem: Vec<Const>,
}

impl Memory {
    pub fn read(&self, x: Var) -> Const {
        self.mem[x.0]
    }

    pub fn write(self, x: Var, v: Const) -> Self {
        self.mem[x.0] = v;
        self
    }
}

struct State {
    label: Label,
    mem: Memory,
}
