use std::time::{Duration, Instant};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::disable_raw_mode,
    terminal::enable_raw_mode,
};

use snake::{Direction, Game};

fn main() {
    enable_raw_mode().expect("Could not enter raw mode");
    let mut game = Game::new();
    game.print();
    loop {
        let start = Instant::now();
        let mut direction = game.get_direction();
        let interval = Duration::from_millis(200);
        while start.elapsed() < interval {
            if let Some(new_direction) = get_input(interval - start.elapsed(), game.get_direction())
            {
                direction = new_direction;
            }
        }
        game.change_direction(direction);
        let game_status = game.tick();
        game.print();
        if game_status.is_none() {
            println!("GAME OVER");
            break;
        } else if game_status.unwrap() {
            println!("YOU WON!");
            break;
        }
    }
    disable_raw_mode().ok();
    println!();
}

fn get_input(timeout: Duration, current_direction: Direction) -> Option<Direction> {
    if !poll(Duration::from_millis(20).min(timeout)).ok()? {
        return None;
    }
    match read().ok()? {
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => Some(Direction::Up),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => Some(Direction::Down),
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            ..
        }) => Some(Direction::Left),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        }) => Some(Direction::Right),
        _ => None,
    }
    .filter(|new_direction| !new_direction.is_opposite(&current_direction))
}
