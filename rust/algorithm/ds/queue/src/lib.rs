
pub mod queue;
pub mod deque;


#[cfg(test)]
mod tests {
    use crate::{queue::Queue, deque::Deque};


    #[test]
    fn queue_push() {
        let mut q: Queue<i32> = Queue::new();
        assert_eq!(q.is_empty(), true);
        for i in 0..10 {
            q.push(i);
        }
        assert_eq!(q.size(), 10);
        assert_eq!(q.is_empty(), false);
        let e = q.front().expect("Get None");
        assert_eq!(*e, 0);
    }

    #[test]
    fn queue_pop() {
        let mut q: Queue<i32> = Queue::new();
        for i in 0..10 {
            q.push(i);
        }
        assert_eq!(q.size(), 10);

        for i in 0..10 {
            assert_eq!(*q.front().unwrap(), i);
            let e = q.pop().expect("get None");
            assert_eq!(e, i);

        }

        assert_eq!(q.size(), 0);
        assert_eq!(q.is_empty(), true);

        if let Some(i) = q.pop() {
            panic!("expect None, get {}", i);
        }
    }

    #[test]
    fn deque_push_back() {
        let mut deq: Deque<i32> = Deque::new();
        assert_eq!(deq.is_empty(), true);
        for i in 0..10 {
            deq.push_back(i);
        }
        assert_eq!(deq.size(), 10);
        assert_eq!(deq.is_empty(), false);
        let e = deq.front().expect("Get None");
        assert_eq!(*e, 0);
        let e = deq.back().expect("Get None");
        assert_eq!(*e, 9);
    }

    #[test]
    fn deque_push_front() {
        let mut deq: Deque<i32> = Deque::new();
        assert_eq!(deq.is_empty(), true);
        for i in 0..10 {
            deq.push_front(i);
        }
        assert_eq!(deq.size(), 10);
        assert_eq!(deq.is_empty(), false);
        let e = deq.front().expect("Get None");
        assert_eq!(*e, 9);
        let e = deq.back().expect("Get None");
        assert_eq!(*e, 0);
    }

    #[test]
    fn queue_pop_front() {
        let mut q: Deque<i32> = Deque::new();
        for i in 0..10 {
            q.push_back(i);
        }
        assert_eq!(q.size(), 10);

        for i in 0..10 {
            assert_eq!(*q.front().unwrap(), i);
            let e = q.pop_front().expect("get None");
            assert_eq!(e, i);
        }

        assert_eq!(q.size(), 0);
        assert_eq!(q.is_empty(), true);

        if let Some(i) = q.pop_front() {
            panic!("expect None, get {}", i);
        }
    }

    #[test]
    fn queue_pop_back() {
        let mut q: Deque<i32> = Deque::new();
        for i in 0..10 {
            q.push_front(i);
        }
        assert_eq!(q.size(), 10);

        for i in 0..10 {
            assert_eq!(*q.back().unwrap(), i);
            let e = q.pop_back().expect("get None");
            assert_eq!(e, i);
        }

        assert_eq!(q.size(), 0);
        assert_eq!(q.is_empty(), true);

        if let Some(i) = q.pop_back() {
            panic!("expect None, get {}", i);
        }
    }
}
