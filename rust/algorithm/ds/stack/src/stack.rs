

pub struct Stack<T> {
    data: Vec<T>,
    top: usize, // 0, if empty
}

impl<T> Stack<T> {

    pub fn new() -> Stack<T> {
        Stack {
            data: Vec::new(),
            top: 0,
        }
    }

    pub fn push(&mut self, e: T) {
        self.data.push(e);
        self.top += 1
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            // in case top overflow
            return None
        }
        self.top = self.top - 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}


