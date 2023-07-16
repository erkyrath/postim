use std::rc::Rc;

struct LendIter<T: Clone> {
    items: Rc<Vec<T>>,
    count: usize,
}

pub struct LendStackIter<T: Clone> {
    stack: Vec<LendIter<T>>,
}

impl<T: Clone> LendStackIter<T> {
    pub fn new(tokens: &Rc<Vec<T>>) -> LendStackIter<T> {
        let first = LendIter {
            items: Rc::clone(tokens),
            count: 0,
        };
        LendStackIter {
            stack: vec![ first ],
        }
    }

    fn next(&mut self) -> Option<T> {
        loop {
            let last = self.stack.last_mut()?;  // or None
            if last.count < last.items.len() {
                let oldcount = last.count;
                last.count += 1;
                return Some(last.items[oldcount].clone());
            }
            else {
                self.stack.pop();
            }
        }
    }
}

