use std::collections::VecDeque;
use priority_queue::PriorityQueue;

#[derive(Hash, PartialEq, Eq)]
enum PlayerEvent {
    WALK{x: i32, y: i32},
}

struct PlayerQueue {
    queue: PriorityQueue<PlayerEvent, i32>,
}

impl PlayerQueue {
    fn new() -> Self { Self { queue: PriorityQueue::new() } }
}

pub struct Player {
    pub username: String,
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub xp: i32,
    pub level: i32,
    pub path: VecDeque<(i32, i32)>,
    queue: PlayerQueue,
}

impl Player {
    pub fn tick(&mut self) {
        // Enqueue the next walk step if there's a path
        if let Some(next_step) = self.path.pop_front() {
            self.queue.queue.push(PlayerEvent::WALK { x: next_step.0, y: next_step.1 }, 0);
        }

        if let Some(event) = self.queue.queue.pop() {
            match event.0 {
                PlayerEvent::WALK { x, y } => {
                    self.x = x;
                    self.y = y;
                },
            }
        }
    }
    pub fn walk(&mut self, x: i32, y: i32) {
        self.queue.queue.push(PlayerEvent::WALK{x, y}, 0);
    }
    pub fn new(username: String) -> Self {
        Self {
            username,
            x: 0,
            y: 0,
            hp: 100,
            max_hp: 100,
            xp: 0,
            level: 1,
            path: VecDeque::new(),
            queue: PlayerQueue::new(),
        }
    }
}
