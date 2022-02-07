use crate::candidate::Candidate;
use crate::number_list::NumberList;
use crate::operation::{Operation, OperationType};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Game {
    pub goal: u32,
    pub numbers: NumberList,
}

impl Game {
    pub fn new(goal: u32, numbers: Vec<u32>) -> Self {
        Self {
            goal,
            numbers: NumberList::new(numbers),
        }
    }
}

struct SolverState {
    candidates: VecDeque<Candidate>,
    seen_lists: HashSet<NumberList>,
    best_distance: u32,
}

impl SolverState {
    fn new(game: Game) -> Self {
        let mut candidates = VecDeque::new();

        candidates.push_front(Candidate::new(game.goal, game.numbers.clone()));

        Self {
            best_distance: game.goal,
            candidates,
            seen_lists: Default::default(),
        }
    }

    fn add_candidate(&mut self, candidate: Candidate) -> bool {
        if self.seen_lists.contains(&candidate.numbers) {
            // We've already seen a candidate solution that arrives to this number list, so no need to further explore it
            return false;
        }

        let dist = candidate.distance().unwrap();

        if dist == 0 {
            println!("WON!!! {candidate}");
            self.best_distance = 0;
            return true;
        } else if dist < self.best_distance {
            println!("FOUND: {candidate}");
            self.best_distance = dist;
        }

        if candidate.numbers.len() > 1 {
            // Only continue if there are 2+ numbers left
            self.seen_lists.insert(candidate.numbers.clone());
            self.candidates.push_back(candidate);
        }
        false
    }
}

pub fn solve(game: Game) {
    let mut state = SolverState::new(game);
    let mut solutions = 0;

    while !state.candidates.is_empty() {
        let base = state.candidates.pop_front().unwrap();

        for (op1, op2) in base.numbers.build_pairs() {
            let c1 = base.with_operation(Operation::new(op1, op2, OperationType::Addition));
            if state.add_candidate(c1) {
                solutions += 1;
            }

            let c2 = base.with_operation(Operation::new(op1, op2, OperationType::Multiplication));
            if state.add_candidate(c2) {
                solutions += 1;
            }

            let subtraction = Operation::new(op1, op2, OperationType::Subtraction);
            if subtraction.is_possible() {
                let c3 = base.with_operation(subtraction);
                if state.add_candidate(c3) {
                    solutions += 1;
                }
            }

            let division = Operation::new(op1, op2, OperationType::Division);
            if division.is_possible() {
                let c4 = base.with_operation(division);
                if state.add_candidate(c4) {
                    solutions += 1;
                }
            }
        }
    }

    println!("Solutions found: {solutions}");
}
