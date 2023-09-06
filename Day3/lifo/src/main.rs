use std::cell::RefCell;

struct StackLifo<T>{
    stack: RefCell<Vec<T>>,
}

impl<T> StackLifo<T>{
    fn new() -> StackLifo<T>{
        StackLifo{stack:RefCell::new(Vec::new())}
    }

    fn push(&mut self, value: T){
        self.stack.borrow_mut().push(value);
    }

    fn pop(&mut self) -> Option<T>{
        self.stack.borrow_mut().pop()
    }
}

fn main() {
    let mut t_stack = StackLifo::new();
    t_stack.push(1);
    t_stack.push(2);
    t_stack.push(3);
    // t_stack.push(4);

    println!("{:?}",t_stack.pop());
    println!("{:?}",t_stack.pop());
    println!("{:?}",t_stack.pop());
    println!("{:?}",t_stack.pop());
}
