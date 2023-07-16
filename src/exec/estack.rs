use std::rc::Rc;

struct LendIter<T> {
    items: Rc<Vec<T>>,
    count: usize,
}

pub struct LendStackIter<T> {
    stack: Vec<LendIter<T>>,
}

impl<T> LendStackIter<T> {
    pub fn new() -> LendStackIter<T> {
        LendStackIter {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, tokens: &Rc<Vec<T>>) {
        let frame = LendIter {
            items: Rc::clone(tokens),
            count: 0,
        };
        self.stack.push(frame);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn next(&mut self) -> Option<&T> {
        loop {
            let last = self.stack.last()?;  // or return None
            if last.count < last.items.len() {
                break;
            }
            else {
                self.stack.pop();
            }
        }
        
        // Previous borrow had to be immutable, but now we need mutable.
        let last = self.stack.last_mut()?;  // or return None
        let oldcount = last.count;
        last.count += 1;
        return Some(&last.items[oldcount]);
    }
}

