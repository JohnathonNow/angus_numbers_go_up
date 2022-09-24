mod server;
mod player;
mod environment;

fn main() {
    println!("Hello, world!");
    let mut s = server::Server::new();
    s.env_queue("hi".into(), 1);
    s.login();
    s.player_walk(0, 3, 3);
    s.tick();
}
