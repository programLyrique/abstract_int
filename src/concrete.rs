// Concrete semantics
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Var(pub usize);

impl Var {
    pub fn new() -> Self {
        static mut n: usize = 0;
        unsafe {
            let var = Var(n);
            n += 1;
            var
        }
    }
}
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Const(pub i64);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Label(usize);

impl Label {
    pub fn new() -> Self {
        Label(0)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinOp {
    #[default]
    Add,
    Sub,
    Mul,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum Expr {
    Const(Const),
    Var(Var),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn new_const(n: i64) -> Self {
        Expr::Const(Const(n))
    }
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

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Cond {
    pub rel: Rel,
    pub left: Var,
    pub right: Const,
}

impl Cond {
    pub fn negate(self: &Cond) -> Cond {
        Cond {
            rel: match self.rel {
                Rel::InfEq => Rel::Sup,
                Rel::Sup => Rel::InfEq,
            },
            left: self.left,
            right: self.right,
        }
    }
}

// TODO: we need to add the labels, but it is not as easy as in Ocaml where one
// can define mutually recursive types.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Command {
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

// Helper functions that computer labels
// (well, dummy now, currently)
impl Command {
    pub fn make_seq(c1: Command, c2: Command) -> Command {
        Self::Seq(Box::new((Label::new(), c1)), Box::new((Label::new(), c2)))
    }

    pub fn make_if(cond: Cond, c1: Command, c2: Option<Command>) -> Command {
        Self::If {
            cond: cond,
            then: Box::new((Label::new(), c1)),
            els: c2.map(|c| Box::new((Label::new(), c))),
        }
    }

    pub fn make_while(cond: Cond, c: Command) -> Command {
        Self::While {
            cond: cond,
            body: Box::new((Label::new(), c)),
        }
    }
    pub fn assign_const(var: Var, n: i64) -> Command {
        Self::Assign(var, Expr::new_const(n))
    }
}

#[macro_export]
macro_rules! seq {
    // The pattern for a seq with 2 elements
    ($e1:expr, $e2:expr) => {
            Command::make_seq($e1, $e2)
    };

    // Decompose multiple `eval`s recursively
    ($e1:expr, $($es:expr),+) => {{
        Command::make_seq($e1, seq! { $($es),+ })
    }};
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Memory {
    mem: Vec<Const>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: vec![Const(0); 100],
        }
    }
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
            Command::Seq(ref c1, ref c2) => self.sem_com(&c1).sem_com(&c2),
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
