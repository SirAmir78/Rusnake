use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
use rand::Rng;

use std::io::{Stdout, Write, stdout};
use std::thread::sleep;
use std::time::Duration;

#[derive(PartialEq)]
enum Status {
    Dead,
    Alive,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Location {
    column: u16,
    row: u16,
}

impl Location {
    fn new(column: u16, row: u16) -> Self {
        Self { column, row }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column && self.row == other.row
    }
}

struct Screen {
    max_column: u16,
    max_row: u16,
}

impl Screen {
    fn new() -> Self {
        let (max_column, max_row) = size().expect("Could not get terminal size");
        Self {
            max_column,
            max_row,
        }
    }
}

struct World {
    screen: Screen,
    snake: Vec<Location>,
    food: Vec<Location>,
    status: Status,
    direction: Direction,
}

impl World {
    fn new() -> Self {
        let screen = Screen::new();

        let mut rng = rand::rng();
        // Random location for snake head
        let head_column = rng.random_range(0..screen.max_column - 1);
        let head_row = rng.random_range(0..screen.max_row - 1);

        let snake = vec![Location {
            column: head_column,
            row: head_row,
        }];

        let status = Status::Alive;
        let direction = Direction::Right;
        let food = Vec::new();

        Self {
            screen,
            snake,
            food,
            status,
            direction,
        }
    }
}

fn add_food(sc: &mut Stdout, world: &mut World) -> std::io::Result<()> {
    let mut rng = rand::rng();

    let is_adding_food = rng.random_range(0..1000);
    if is_adding_food > 950 {
        let food_location = Location::new(
            rng.random_range(0..world.screen.max_column - 1),
            rng.random_range(0..world.screen.max_row - 1),
        );
        world.food.push(food_location);
    }
    Ok(())
}

fn draw(sc: &mut Stdout, world: &mut World) -> std::io::Result<()> {
    // Clear screen
    sc.queue(Clear(ClearType::All))?;

    // Draw the food
    for part in &world.food {
        sc.queue(MoveTo(part.column, part.row))?;
        sc.queue(Print("F"))?;
    }

    // Draw the snake
    for part in &world.snake {
        sc.queue(MoveTo(part.column, part.row))?;
        sc.queue(Print("O"))?;
    }

    sc.flush()?;
    Ok(())
}

fn grow_snake(sc: &mut Stdout, world: &mut World) -> std::io::Result<()> {
    let head = world.snake[0];
    let tail = world.snake.last().unwrap();

    let mut food_eaten = false;
    for (index, part) in world.food.iter().enumerate() {
        if part.column == head.column && part.row == head.row {
            food_eaten = true;
            world.food.remove(index);
            break;
        }
    }

    if food_eaten == true {
        let new_tail = match world.direction {
            Direction::Up => Location {
                column: tail.column,
                row: tail.row + 1,
            },
            Direction::Down => Location {
                column: tail.column,
                row: tail.row.saturating_sub(1),
            },
            Direction::Left => Location {
                column: tail.column + 1,
                row: tail.row,
            },
            Direction::Right => Location {
                column: tail.column.saturating_sub(1),
                row: tail.row,
            },
        };

        world.snake.push(new_tail);
    }

    Ok(())
}

fn check_keyboard_input(world: &mut World) {
    if poll(Duration::from_millis(50)).expect("Failed to poll event") {
        if let Event::Key(KeyEvent { code, .. }) = read().expect("Failed to read event") {
            match code {
                KeyCode::Up => {
                    if world.direction != Direction::Down {
                        world.direction = Direction::Up;
                    }
                }
                KeyCode::Down => {
                    if world.direction != Direction::Up {
                        world.direction = Direction::Down;
                    }
                }
                KeyCode::Left => {
                    if world.direction != Direction::Right {
                        world.direction = Direction::Left;
                    }
                }
                KeyCode::Right => {
                    if world.direction != Direction::Left {
                        world.direction = Direction::Right;
                    };
                }
                KeyCode::Char('q') => {
                    world.status = Status::Dead;
                    println!("👋 Quitting!");
                }
                _ => {}
            }
        }
    }
}

fn physics(world: &mut World) {
    let mut head = world.snake[0];

    match world.direction {
        Direction::Up => {
            if head.row > 0 {
                head.row -= 1;
            } else {
                head.row = world.screen.max_row
            }
        }
        Direction::Down => {
            if head.row < world.screen.max_row {
                head.row += 1;
            } else {
                head.row = 0
            }
        }
        Direction::Left => {
            if head.column > 0 {
                head.column -= 1;
            } else {
                head.column = world.screen.max_column
            }
        }
        Direction::Right => {
            if head.column < world.screen.max_column {
                head.column += 1;
            } else {
                head.column = 0
            }
        }
    }

    if world.snake.contains(&head) {
        world.status = Status::Dead;
    } else {
        world.snake.insert(0, head);
        world.snake.pop();
    }
}

fn main() -> std::io::Result<()> {
    let mut sc = stdout();
    sc.execute(Hide)?;
    enable_raw_mode()?;
    execute!(sc, Clear(ClearType::All))?;
    let mut world = World::new();
    let up_down_game_speed = world.screen.max_column;
    let left_right_game_speed = world.screen.max_row;
    while world.status == Status::Alive {
        draw(&mut sc, &mut world)?;
        add_food(&mut sc, &mut world)?;
        check_keyboard_input(&mut world);
        grow_snake(&mut sc, &mut world)?;
        physics(&mut world);

        if world.direction == Direction::Up || world.direction == Direction::Down {
            sleep(Duration::from_millis(up_down_game_speed as u64));
        } else if world.direction == Direction::Right || world.direction == Direction::Left {
            sleep(Duration::from_millis(left_right_game_speed as u64));
        }
    }
    sleep(Duration::from_secs(3));
    sc.execute(Clear(ClearType::All))?;
    sc.queue(MoveTo(
        world.screen.max_column / 2,
        world.screen.max_row - 4,
    ))?;
    sc.queue(Print("Game Over..."))?;
    sc.flush()?;
    sc.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
