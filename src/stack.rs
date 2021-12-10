#[derive(Clone, Default)]
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    fn setup_stack<T>(v: Vec<T>) -> Stack<T> {
        Stack(v)
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<u32> = Stack::default();
        stack.push(34);
        assert_eq!(stack.0, vec![34]);
    }

    #[test]
    fn test_pop_empty() {
        let mut stack: Stack<u32> = Stack::default();
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn test_pop_with_data() {
        let mut stack: Stack<String> = setup_stack(vec!["a".to_string(), "b".to_string()]);
        let item = stack.pop();
        assert_eq!(Some("b".to_string()), item);
        assert_eq!(stack.0, vec!["a".to_string()]);
    }

    #[test]
    fn test_peek_empty() {
        let stack: Stack<String> = Stack::default();
        assert_eq!(None, stack.peek());
    }

    #[test]
    fn test_peek_with_data() {
        let stack: Stack<String> = setup_stack(vec!["a".to_string(), "b".to_string()]);
        let item = stack.peek();
        assert_eq!(Some(&"b".to_string()), item);
        assert_eq!(stack.0, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_size() {
        let stack: Stack<bool> = setup_stack(vec![true, false, false, true]);
        assert_eq!(4, stack.size());
    }
}
