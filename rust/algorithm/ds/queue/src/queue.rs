
pub struct Queue<T> {
    data: Vec<T>,
    size: usize,
}


impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, e: T) {
        self.size += 1;
        self.data.push(e);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None
        }
        self.size -= 1;
        Some(self.data.remove(0))
    }

    pub fn is_empty(& self) -> bool {
        self.size == 0
    }

    pub fn front(& self) -> Option<&T>{
        self.data.get(0)
    }
    
    pub fn size(& self) -> usize {
        self.size
    }
}

