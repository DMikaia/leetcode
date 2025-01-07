struct MinStack {
    stack: Vec<Element>,
    min: i32,
}

struct Element {
    val: i32,
    min: i32
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: i32::MAX,
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(Element { val, min: self.min });
        self.min = val.min(self.min);
    }
    
    fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            self.min = val.min;
        } 
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */