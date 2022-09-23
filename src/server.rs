use crate::{player::Player, environment::{Environment, EnvironmentEvent}};

pub struct Server {
    env: Environment,
    players: Vec<Player>,
}

impl Server {
    pub fn new() -> Self {
        Self { env: Environment::new(), players: Vec::new() } 
        
    }
    
    pub fn env_queue(&mut self, s: String, i: i32) {
        self.env.enqueue(EnvironmentEvent::MESSAGE{text: s}, i);
    }

    pub fn tick(&mut self) {
        self.env.tick();
        for player in &mut self.players {
            player.tick();
        }
    }
}
