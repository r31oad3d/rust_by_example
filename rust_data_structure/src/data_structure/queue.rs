#[derive(Debug)]
pub struct Queue<T> {
    qdata: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { qdata: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.qdata.push(val);
    }
    pub fn pop(&mut self) -> Option<T> {
        let val = self.qdata.pop();
        match val {
            None => None,
            Some(t) => Some(t),
        }
    }
}
