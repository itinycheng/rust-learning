pub struct Queue<T> {
    queue: Vec<T>
}

 impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.queue.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop()
    }
}