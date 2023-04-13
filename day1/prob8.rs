struct Queue {
    data: Vec<i32>,
}

impl Queue {
    fn new() -> Queue {
        Queue { data: Vec::new() }
    }

    fn enqueue(&mut self, value: i32) {
        self.data.push(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }else{
            return Some(self.data.remove(0));
        } 
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}

fn main() {
    let mut q = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.is_empty(), false);
    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), None);
    assert_eq!(q.is_empty(), true);
}
