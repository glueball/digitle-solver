use crate::operation::Operation;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct NumberList {
    numbers: Vec<u32>,
}

impl NumberList {
    pub fn new(mut numbers: Vec<u32>) -> Self {
        numbers.sort();

        Self { numbers }
    }

    pub fn build_pairs(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        NumberListPairIterator::new(&self.numbers)
    }

    pub fn substitute_operation(&self, operation: &Operation) -> Self {
        let mut found1 = false;
        let mut found2 = false;

        let mut numbers = Vec::with_capacity(self.numbers.len() - 1);
        numbers.push(operation.result().unwrap());

        for item in &self.numbers {
            if !found1 && *item == operation.op1 {
                found1 = true;
            } else if !found2 && *item == operation.op2 {
                found2 = true;
            } else {
                numbers.push(*item)
            }
        }

        numbers.sort();

        Self { numbers }
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }
}

struct NumberListPairIterator<'a> {
    numbers: &'a Vec<u32>,
    i: usize,
    j: usize,
    done: bool,
}

impl<'a> NumberListPairIterator<'a> {
    fn new(numbers: &'a Vec<u32>) -> Self {
        Self {
            numbers,
            i: 0,
            j: 1,
            done: false,
        }
    }

    fn advance_j(&mut self) {
        loop {
            self.j += 1;

            if self.j > self.numbers.len() - 1 {
                return self.advance_i();
            }

            if self.numbers[self.j] != self.numbers[self.j - 1] {
                return;
            }
        }
    }

    fn advance_i(&mut self) {
        loop {
            self.i += 1;
            self.j = self.i + 1;

            if self.i > self.numbers.len() - 2 {
                self.done = true;
                return;
            }

            if self.numbers[self.i] != self.numbers[self.i - 1] {
                return;
            }
        }
    }
}

impl<'a> Iterator for NumberListPairIterator<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.j > self.numbers.len() - 1 {
            return None;
        }

        let out = Some((self.numbers[self.j], self.numbers[self.i]));

        self.advance_j();

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::number_list::NumberList;
    use crate::solve::*;

    #[test]
    fn build_paris() {
        let numbers = NumberList::new(vec![50, 25, 10, 6, 5, 10]);
        // 5, 6, 10, 10, 25, 50

        let expected = vec![
            (6, 5),
            (10, 5),
            (25, 5),
            (50, 5),
            (10, 6),
            (25, 6),
            (50, 6),
            (10, 10),
            (25, 10),
            (50, 10),
            (50, 25),
        ];

        assert_eq!(expected, numbers.build_pairs().collect::<Vec<(u32, u32)>>());
    }

    #[test]
    fn build_paris_2() {
        let numbers = NumberList::new(vec![1, 1, 1]);

        assert_eq!(
            vec![(1, 1)],
            numbers.build_pairs().collect::<Vec<(u32, u32)>>()
        );

        let numbers = NumberList::new(vec![1, 1, 1, 2]);

        assert_eq!(
            vec![(1, 1), (2, 1)],
            numbers.build_pairs().collect::<Vec<(u32, u32)>>()
        );
    }
}
