//! Design Circular Dequeue

struct MyCircularDeque {
    v: Vec<i32>,
    front: usize,
    back: usize,
    size: usize,
    capacity: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            v: vec![-1; k ],
            front: 0,
            back: k - 1,
            size: 0,
            capacity: k,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.capacity - 1) % self.capacity;
        self.v[self.front] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.back = (self.back + 1) % self.capacity;
        self.v[self.back] = value;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.back = (self.back + self.capacity - 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.v[self.front]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.v[self.back]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}


pub fn run_circular_dequeue () {
    let mut obj = MyCircularDeque::new(3);
    let ret_1 = obj.insert_last(1);
    let ret_2 = obj.insert_last(2);
    let ret_3 = obj.insert_front(3);
    let ret_4 = obj.insert_front(4);
    let ret_5 = obj.get_rear();
    let ret_6 = obj.is_full();
    let ret_7 = obj.delete_last();
    let ret_8 = obj.insert_front(4);
    let ret_9 = obj.get_front();

    println!("{ret_1} and {ret_2} and {ret_3} and {ret_4} and {ret_5} and {ret_6} and {ret_7} and {ret_8} and {ret_9}");
}

/*
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */