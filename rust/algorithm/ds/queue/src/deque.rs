
pub struct Deque<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0,
        }
    }

    pub fn push_back(&mut self, e: T) {
        self.size += 1;
        self.data.push(e);
    }

    pub fn push_front(&mut self, e: T) {
        self.size += 1;
        self.data.insert(0, e);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 {
            return None
        }
        self.size -= 1;
        Some(self.data.remove(0))
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            return None
        }
        self.size -= 1;
        self.data.pop()
    }

    pub fn is_empty(& self) -> bool {
        self.size == 0
    }

    pub fn front(& self) -> Option<&T>{
        self.data.get(0)
    }
    
    pub fn back(& self) -> Option<&T>{
        self.data.last()
    }

    pub fn size(& self) -> usize {
        self.size
    }
}

