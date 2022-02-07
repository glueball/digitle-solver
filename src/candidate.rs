use crate::number_list::NumberList;
use crate::operation::Operation;
use std::fmt::{Display, Formatter};

pub struct Candidate {
    pub numbers: NumberList,
    pub ops: Vec<Operation>,
    pub goal: u32,
}

impl Candidate {
    pub fn new(goal: u32, numbers: NumberList) -> Self {
        Self {
            ops: Vec::new(),
            numbers,
            goal,
        }
    }

    pub fn result(&self) -> Option<u32> {
        self.ops.last().and_then(|op| op.result())
    }

    pub fn distance(&self) -> Option<u32> {
        self.result().map(|result| {
            if result > self.goal {
                result - self.goal
            } else {
                self.goal - result
            }
        })
    }

    pub fn with_operation(&self, op: Operation) -> Self {
        if !op.is_possible() {
            panic!("Candidates cannot have unsolvable operations")
        }

        let numbers = self.numbers.substitute_operation(&op);
        let mut ops = self.ops.clone();
        ops.push(op);

        Self {
            ops,
            numbers,
            goal: self.goal,
        }
    }
}

impl Display for Candidate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for op in &self.ops {
            write!(f, "{op}, ")?;
        }

        if let Some(dist) = self.distance() {
            if dist == 0 {
                write!(f, " ***")
            } else if dist <= 5 {
                write!(f, "[dist = {dist}] **")
            } else if dist <= 10 {
                write!(f, "[dist = {dist}] *")
            } else {
                write!(f, "[dist = {dist}]")
            }
        } else {
            write!(f, "[No distance]")
        }
    }
}
