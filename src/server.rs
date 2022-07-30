use priority_queue::PriorityQueue;

pub struct Server {
    env: PriorityQueue<String, i32>
}

impl Server {
    pub fn new() -> Self {
        Self { env: PriorityQueue::new() } 
    }
    pub fn env_queue(&mut self, s: String, i: i32) {
        self.env.push(s, i);
    }
}
