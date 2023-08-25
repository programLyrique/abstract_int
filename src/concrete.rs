// Concrete semantics
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Var(usize);
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Const(pub i64);
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Label(usize);

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum BinOp {
    #[default]
    Add,
    Sub,
    Mul,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Rel {
    #[default]
    InfEq,
    Sup,
}

pub fn relop(c: Rel, left: Const, right: Const) -> bool {
    match c {
        Rel::InfEq => left.0 <= right.0,
        Rel::Sup => left.0 > right.0,
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Expr {
    Const(Const),
    Var(Var),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Const(Const(0))
    }
}

pub fn binop(op: BinOp, left: Const, right: Const) -> Const {
    Const(match op {
        BinOp::Add => left.0 + right.0,
        BinOp::Sub => left.0 - right.0,
        BinOp::Mul => left.0 * right.0,
    })
}

#[derive(Default, PartialEq, Eq, Clone)]
struct Cond {
    rel: Rel,
    left: Var,
    right: Const,
}

// TODO: we need to add the labels, but it is not as easy as in Ocaml where one
// can define mutually recursive types.
#[derive(Default, PartialEq, Eq, Clone)]
enum Command {
    #[default]
    Skip,
    Seq(Box<(Label, Command)>, Box<(Label, Command)>),
    Assign(Var, Expr),
    Input(Var),
    If {
        cond: Cond,
        then: Box<(Label, Command)>,
        els: Option<Box<(Label, Command)>>,
    },
    While {
        cond: Cond,
        body: Box<(Label, Command)>,
    },
}

#[derive(Default, PartialEq, Eq, Clone)]
struct Memory {
    mem: Vec<Const>,
}

impl Memory {
    pub fn read(&self, x: Var) -> Const {
        self.mem[x.0]
    }

    pub fn write(mut self, x: Var, v: Const) -> Self {
        self.mem[x.0] = v;
        self
    }

    pub fn sem_expr(&self, e: &Expr) -> Const {
        match e {
            Expr::Const(c) => *c,
            Expr::Var(i) => self.read(*i),
            Expr::BinOp { op, left, right } => {
                binop(*op, self.sem_expr(&left), self.sem_expr(&right))
            }
        }
    }

    pub fn sem_cond(&self, c: &Cond) -> bool {
        relop(c.rel, self.read(c.left), c.right)
    }

    // Denotational semantics
    pub fn sem_com(self, state: &(Label, Command)) -> Self {
        match state.1 {
            Command::Skip => self,
            Command::Seq(ref c1, ref c2) => self.sem_com(&c2).sem_com(&c1),
            Command::Assign(x, ref expr) => {
                let res = self.sem_expr(&expr);
                self.write(x, res)
            }
            Command::Input(x) => self.write(x, input()), //TODO: input
            Command::If {
                ref cond,
                ref then,
                ref els,
            } => {
                if self.sem_cond(&cond) {
                    self.sem_com(&then)
                } else {
                    if els.is_none() {
                        // would like to do it with map_or_else but it creates borrowing problems
                        self
                    } else {
                        self.sem_com(els.as_ref().unwrap())
                    }
                }
            }
            Command::While { ref cond, ref body } => {
                if self.sem_cond(&cond) {
                    self.sem_com(&body).sem_com(state)
                } else {
                    self.clone()
                }
            }
        }
    }
}

pub fn input() -> Const {
    Const(0) // TODO: actually get an input from stdin
}

#[derive(Default, PartialEq, Eq)]
pub struct State {
    label: Label,
    mem: Memory,
}

// transitional semantics
// TODO
