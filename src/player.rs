use priority_queue::PriorityQueue;

#[derive(Hash, PartialEq, Eq)]
enum PlayerEvent {
    WALK{x: i32, y: i32},
}

struct PlayerQueue {
    queue: PriorityQueue<PlayerEvent, i32>,
}

pub struct Player {
    x: i32,
    y: i32,
    queue: PlayerQueue,
}

impl Player {
    pub fn tick(&mut self) {
        
    }
}
