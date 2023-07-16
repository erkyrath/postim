use std::rc::Rc;

struct LendIter<T> {
    items: Rc<Vec<T>>,
    count: usize,
}

pub struct LendStackIter<T> {
    stack: Vec<LendIter<T>>,
}

impl<T> LendStackIter<T> {
    pub fn new(tokens: &Rc<Vec<T>>) -> LendStackIter<T> {
        let first = LendIter {
            items: Rc::clone(tokens),
            count: 0,
        };
        LendStackIter {
            stack: vec![ first ],
        }
    }

    fn next(&mut self) -> Option<&T> {
        if self.stack.len() > 0 {
            if self.stack[0].items.len() > 0 {
                return Some(&self.stack[0].items[0]);
            }
        }
        None
    }
}

