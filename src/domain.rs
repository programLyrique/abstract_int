// Abstract domains
use concrete::Var;

// Non-relational abstraction

enum AbstractValue {
    Bottom,
    Top,
    Positive,
    Negative,
}

pub fn includes(a1: AbstractValue, a2: AbstractValue) -> bool {
    a1 == AbstractValue::Bottom || a2 == AbstractValue::Top || a1 == a2
}

struct AbstractDomain {
    domain: Vec<abstract_value>,
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
