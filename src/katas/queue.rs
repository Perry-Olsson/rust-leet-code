use std::{collections::VecDeque, i32};

struct MyStack {
    queue: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        MyStack {
            queue: VecDeque::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.queue.push_back(x)
    }
    
    fn pop(&mut self) -> i32 {
        let queue_length = self.queue.len();
        let mut popped_value = i32::MIN;
        for i in 0..queue_length {
            popped_value = self.queue.pop_front().unwrap();
            if i != queue_length - 1 {
                self.queue.push_back(popped_value);
            }
        }
        popped_value
    }
    
    fn top(&mut self) -> i32 {
        let queue_length = self.queue.len();
        let mut popped_value = i32::MIN;
        for _ in 0..queue_length {
            popped_value = self.queue.pop_front().unwrap();
            self.queue.push_back(popped_value);
        }
        popped_value
    }
    
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
