#[derive(Debug)]
struct MyQueue {
    pub stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);
        let mut i: usize = self.stack.len() - 1;
        while i != 0 {
            self.stack.swap(i, i - 1);
            i -= 1;
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.stack.len() == 0 {
            panic!("stack is empty");
        }
        return self.stack.pop().unwrap();
    }
    
    fn peek(&self) -> i32 {
        if self.stack.len() == 0 {
            panic!("stack is empty");
        }
        return  self.stack[self.stack.len() - 1];
    }
    
    fn empty(&self) -> bool {
        return self.stack.len() == 0;
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */


fn main() {
    let mut obj = MyQueue::new();
    println!("obj = {:?}", obj.peek());
}