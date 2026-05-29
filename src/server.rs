
use std::collections::{HashMap, HashSet, VecDeque};
use crate::{player::Player, environment::{Environment, EnvironmentEvent}, packets::PlayerState};

pub struct Map {
    pub blocked_tiles: HashSet<(i32, i32)>,
}

impl Map {
    pub fn new() -> Self {
        let mut blocked_tiles = HashSet::new();
        // Hardcode some terrain
        for x in 3..=7 {
            blocked_tiles.insert((x, 5));
        }
        Self { blocked_tiles }
    }

    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.blocked_tiles.contains(&(x, y))
    }
}

pub struct Server {
    pub tick_count: i64,
    env: Environment,
    pub players: HashMap<String, Player>,
    map: Map,
}

impl Server {
    pub fn new() -> Self {
        Self {
            tick_count: 0,
            env: Environment::new(),
            players: HashMap::new(),
            map: Map::new(),
        }
    }
    
    pub fn env_queue(&mut self, s: String, i: i32) {
        self.env.enqueue(EnvironmentEvent::MESSAGE{text: s}, i);
    }

    pub fn login(&mut self, username: String) {
        self.players.insert(username.clone(), Player::new(username));
    }

    pub fn logout(&mut self, username: &str) {
        self.players.remove(username);
    }

    pub fn player_click_walk(&mut self, username: &str, target_x: i32, target_y: i32) {
        if let Some(player) = self.players.get_mut(username) {
            let start = (player.x, player.y);
            let goal = (target_x, target_y);

            if self.map.is_blocked(goal.0, goal.1) {
                return; // Target is blocked
            }

            // Simple BFS for pathfinding with max search depth
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let mut came_from = HashMap::new();

            queue.push_back(start);
            visited.insert(start);

            let mut found = false;
            let mut iterations = 0;
            let max_iterations = 1000;

            while let Some(current) = queue.pop_front() {
                if current == goal {
                    found = true;
                    break;
                }

                iterations += 1;
                if iterations >= max_iterations {
                    break;
                }

                let neighbors = [
                    (current.0 + 1, current.1),
                    (current.0 - 1, current.1),
                    (current.0, current.1 + 1),
                    (current.0, current.1 - 1),
                ];

                for next in neighbors.iter() {
                    if !visited.contains(next) && !self.map.is_blocked(next.0, next.1) {
                        visited.insert(*next);
                        came_from.insert(*next, current);
                        queue.push_back(*next);
                    }
                }
            }

            if found {
                let mut path = Vec::new();
                let mut current = goal;
                while current != start {
                    path.push(current);
                    current = *came_from.get(&current).unwrap();
                }
                path.reverse();
                player.path = path.into();
            }
        }
    }

    pub fn get_state(&self) -> Vec<PlayerState> {
        self.players.values().map(|p| PlayerState {
            username: p.username.clone(),
            x: p.x,
            y: p.y,
            hp: p.hp,
            max_hp: p.max_hp,
            xp: p.xp,
            level: p.level,
        }).collect()
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.env.tick();
        for player in self.players.values_mut() {
            player.tick();
        }
    }
}
