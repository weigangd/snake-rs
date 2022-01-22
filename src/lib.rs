use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use rand::seq::IteratorRandom;
use std::io::stdout;

const N: usize = 10;
const N2: usize = N + 2;
const N_POINTS: usize = N2 * N2;

pub struct Field {
    field: [isize; N_POINTS],
}

impl Field {
    pub fn new() -> Self {
        let mut field = [0; N_POINTS];
        for p in field[..N2].iter_mut() {
            *p = isize::MAX;
        }
        for row in field[N2..N_POINTS - N2].chunks_exact_mut(N2) {
            row[0] = isize::MAX;
            row[N2 - 1] = isize::MAX;
        }
        for p in field[N_POINTS - N2..].iter_mut() {
            *p = isize::MAX;
        }
        Self { field }
    }

    pub fn make_turn(&mut self) {
        for point in self.field.iter_mut() {
            *point = point.saturating_sub(1);
        }
    }

    pub fn set_point(&mut self, position: usize, value: isize) {
        self.field[position] = value;
    }

    pub fn is_valid(&self, position: usize) -> bool {
        !self.field[position].is_positive()
    }
    pub fn is_fruit(&self, position: usize) -> bool {
        self.field[position] == isize::MIN
    }

    pub fn print(&self) {
        let mut stdout = stdout();
        stdout.queue(cursor::Hide).unwrap();
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        print!("\u{250F}");
        for _ in 1..N2 - 1 {
            print!("\u{2501}\u{2501}");
        }
        print!("\u{2501}\u{2513}");
        println!();
        for (i, row) in self.field[N2..N_POINTS - N2].chunks_exact(N2).enumerate() {
            stdout.queue(cursor::MoveTo(0, (i + 1) as u16)).unwrap();
            print!("\u{2503} ");
            for i in &row[1..N2 - 1] {
                if i.is_positive() {
                    print!("\u{25A0} ");
                } else {
                    if *i == isize::MIN {
                        print!("\x1b[1;31m\u{25A0}\x1b[1;37m ");
                    } else {
                        print!("  ");
                    }
                }
            }
            println!("\u{2503}");
        }
        stdout.queue(cursor::MoveTo(0, (N2 - 1) as u16)).unwrap();
        print!("\u{2517}");
        for _ in 1..N2 - 1 {
            print!("\u{2501}\u{2501}");
        }
        print!("\u{2501}\u{251B}");
        println!("");
        stdout.queue(cursor::MoveTo(0, N2 as u16)).unwrap();
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposite(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Up, Self::Down) => true,
            (Self::Down, Self::Up) => true,
            (Self::Left, Self::Right) => true,
            (Self::Right, Self::Left) => true,
            _ => false,
        }
    }
}

pub struct Snake {
    direction: Direction,
    length: isize,
    position: usize,
}

impl Snake {
    fn new() -> Self {
        Snake {
            direction: Direction::Down,
            length: 2,
            position: N2 + 1,
        }
    }

    fn make_move(&mut self) {
        match self.direction {
            Direction::Up => self.position -= N2,
            Direction::Down => self.position += N2,
            Direction::Left => self.position -= 1,
            Direction::Right => self.position += 1,
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        if !self.direction.is_opposite(&direction) {
            self.direction = direction;
        }
    }

    fn increase_length(&mut self) {
        self.length += 1;
    }
}

pub struct Game {
    field: Field,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        let snake = Snake::new();
        let mut field = Field::new();
        field.set_point(snake.position, snake.length);
        let mut game = Game {
            field,
            snake: Snake::new(),
        };
        game.spawn_fruit();
        game
    }

    pub fn tick(&mut self) -> bool {
        self.snake.make_move();
        let pos = self.snake.position;
        if self.field.is_valid(pos) {
            if self.field.is_fruit(pos) {
                self.snake.increase_length();
                self.field.set_point(pos, self.snake.length);
                self.spawn_fruit();
            } else {
                self.field.make_turn();
                self.field.set_point(pos, self.snake.length);
            }
            false
        } else {
            true
        }
    }

    pub fn print(&self) {
        self.field.print();
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.change_direction(direction);
    }
    pub fn get_direction(&self) -> Direction {
        self.snake.direction
    }

    fn spawn_fruit(&mut self) {
        let mut rng = rand::thread_rng();
        let fruit = self
            .field
            .field
            .iter_mut()
            .filter(|v| !v.is_positive())
            .choose(&mut rng)
            .expect("game won");
        *fruit = isize::MIN;
    }
}
