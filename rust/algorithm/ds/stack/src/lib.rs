pub mod stack;


#[cfg(test)]
mod tests {
    use crate::stack::Stack;


    #[test]
    fn stack_push() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.is_empty(), true);
        for i in 0..10 {
            stack.push(i);
        }
        assert_eq!(stack.size(), 10);
        assert_eq!(stack.is_empty(), false);

    }

    #[test]
    fn stack_pop() {
        let mut stack: Stack<i32> = Stack::new();
        for i in 0..10 {
            stack.push(i);
        }
        assert_eq!(stack.size(), 10);

        let mut i = 9;
        while i >= 0  {
            assert_eq!(*stack.peek().unwrap(), i);

            let e = stack.pop().expect("get None");
            assert_eq!(e, i);
            i -= 1;
        }

        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);

        if let Some(i) = stack.pop() {
            panic!("expect None, get {}", i);
        }
    }
}
