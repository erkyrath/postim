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
        loop {
            let last = self.stack.last()?;  // or None
            if last.count >= last.items.len() {
                self.stack.pop();
                continue;
            }

            // Previous borrow had to be immutable, but now we need mutable.
            let lastm = self.stack.last_mut()?;  // or None
            let oldcount = lastm.count;
            lastm.count += 1;
            return Some(&lastm.items[oldcount]);
        }
    }
}

