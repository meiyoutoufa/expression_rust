pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: vec![] }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn last(&self) -> Option<&T> {
        self.items.last()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
}