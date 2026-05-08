use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quote {
    pub bid: f64,
    pub ask: f64,
}

#[derive(Debug, Clone)]
pub struct ScratchBuffer {
    data: Vec<f64>,
}

impl ScratchBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn clear_and_extend(&mut self, values: &[f64]) -> &[f64] {
        self.data.clear();
        self.data.extend_from_slice(values);
        &self.data
    }
}

pub struct DropCounter {
    drops: Rc<Cell<usize>>,
}

impl DropCounter {
    pub fn new(drops: Rc<Cell<usize>>) -> Self {
        Self { drops }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_types_are_duplicated_without_moving_ownership() {
        let quote = Quote {
            bid: 100.0,
            ask: 100.1,
        };
        let copied = quote;

        assert_eq!(quote, copied);
    }

    #[test]
    fn raii_runs_cleanup_when_owner_goes_out_of_scope() {
        let drops = Rc::new(Cell::new(0));
        {
            let _counter = DropCounter::new(Rc::clone(&drops));
            assert_eq!(drops.get(), 0);
        }

        assert_eq!(drops.get(), 1);
    }
}
