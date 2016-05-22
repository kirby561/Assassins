use std::collections::LinkedList;

mod application;
use application::App;
use application::AppState;

struct Server {
    name: String,
}

struct GpsLocation {
    latitude: i64,
    longitude: i64,
}

struct Game {
    unique_id: i64,
    time_limit: i64,
    elapsed_time: i64,
    server: Server,
    players: LinkedList<Player>,
}

struct Player {
    name: String,
    icon_path: String,
    target: Option<Box<Player>>,
    targeting_players: LinkedList<Box<Player>>,
    location: GpsLocation,
    score: i32,
    state: PlayerState,
}

#[derive(Debug)]
enum PlayerState {
    Alive,
    Dead,
}

fn main() {
    // Make some things
    let game = Game {
        unique_id: 999,
        time_limit: 10000,
        elapsed_time: 0,
        server: Server { name: String::from("ServerName") },
        players: LinkedList::new(),
    };

    let player1 = Player {
        name: String::from("Player 1"),
        icon_path: String::from("/whatever/nopathyet"),
        target: None,
        targeting_players: LinkedList::new(),
        location: GpsLocation {
            latitude: 0,
            longitude: 0,
        },
        score: 0,
        state: PlayerState::Dead,
    };

    let mut app = App::new();
    app.run();

    println!("App closing.");
}
