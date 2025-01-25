// IMPORTS
use std::io::Write;
use std::{thread, time};
use k_board::{keyboard::Keyboard, keys::Keys};
use termion::terminal_size;
use rand::Rng;

// Pair Struct
#[derive(Clone, Copy)]
struct Pair {
    x: i32,
    y: i32,
} 

// Object Struct
#[derive(Clone, Copy)]
struct Object {
    pos: Pair,
    dir: Pair,
    chr: char,
}

// Screen Struct
struct Screen {
    width: u32,
    height: u32,
    padx: u32,
    pady: u32,
    header: String,
    footer: String,
}

impl Screen {

    // New.
    fn new() -> Self {
        // Get terminal size using termion
        let (term_width, term_height) = terminal_size().unwrap_or((80, 24)); // Fallback to (80, 24) if unable to get size

        // Calculate 80% of terminal width and height
        let width = (term_width as f32 * 0.8) as u32;
        let height = (term_height as f32 * 0.8) as u32;

        // Calculate padding to center the screen
        let padx = (term_width as u32 - (width + 2)) / 2;
        let pady = (term_height as u32 - (height + 2)) / 2;

        // Create and return a new Screen instance
        Screen {
            width,
            height,
            padx,
            pady,
            header: String::new(),
            footer: String::new(),
        }
    }

    // Clear screen fn.
    fn clear_screen() {
        print!("\x1B[2J\x1B[H");
    }

    // Render fn.
    fn render(&self, objects: &Vec<Object>) {

        // O/P String.
        let mut output: String = String::new();

        // pady.
        output.push_str(&"\n".repeat(self.pady as usize));

        // Header.
        let header_padding = (self.width as usize - self.header.len()) / 2;
        let remaining_padding = self.width as usize - (header_padding + self.header.len());
        output.push_str(&" ".repeat(self.padx as usize));
        output.push_str("┌"); 
        output.push_str(&"─".repeat(header_padding));
        output.push_str(&self.header);
        output.push_str(&"─".repeat(remaining_padding));
        output.push_str("┐\n");

        // Screen.
        for y in 0..self.height {

            output.push_str(&" ".repeat(self.padx as usize));
            output.push_str("│");
            for x in 0..self.width {
                let mut printed = false;
                for obj in objects {
                    if obj.pos.x == x as i32 && obj.pos.y == y as i32 {
                        output.push_str(&String::from(obj.chr));
                        printed = true;
                        break;
                    }
                }
                if !printed {
                    output.push_str(" ");
                }
            }
            output.push_str("│\n");
        }

        // Footer.
        let footer_padding = (self.width as usize - self.footer.len()) / 2;
        let remaining_padding = self.width as usize - (footer_padding + self.footer.len());
        output.push_str(&" ".repeat(self.padx as usize));
        output.push_str("└");
        output.push_str(&"─".repeat(footer_padding));
        output.push_str(&self.footer);
        output.push_str(&"─".repeat(remaining_padding));
        output.push_str("┘\n");

        // pady.
        output.push_str(&"\n".repeat(self.pady as usize));

        Screen::clear_screen();
        print!("{}", output);
        std::io::stdout().flush().unwrap();
    }
}

// Main fn.
fn main() {

    let mut screen = Screen::new();

    screen.header = String::from(" SNAKE-TTY ");

    let fps: u64 = 60;
    let sleep_duration: time::Duration = time::Duration::from_millis(1000 / fps); 
    let mut rng = rand::thread_rng();

    let mut score: u32 = 0;

    let mut snake: Vec<Object> = vec![
        Object {
            pos: Pair { x: (rng.gen_range(0..screen.width) as i32), y: (rng.gen_range(0..screen.height) as i32)},
            dir: Pair { x: 1, y: 0},
            chr: 'X'
        }
    ]; 

    let mut food: Object = Object {
        pos: Pair { x: (rng.gen_range(0..screen.width) as i32), y: (rng.gen_range(0..screen.height) as i32)},
        dir: Pair { x: 1, y: 0},
        chr: 'O'
    };

    let mut first_hit: bool = false;
    let mut is_running: bool = true;
    let mut game_is_on: bool = false;

    while is_running {

        // Objects to be rendered.
        let mut objects: Vec<Object> = vec![];

        if game_is_on {

            // Body movement.
            let mut index = snake.len() - 1;
            while index > 0 {
                snake[index].pos = snake[index - 1].pos;
                snake[index].dir = snake[index - 1].dir;
                index -= 1;
            }

            // Head movement.
            let snake_head: &mut Object = &mut snake[0];

            for key in Keyboard::new() {
                match key {
                    Keys::Escape => {
                        is_running = false;
                        break;
                    },
                    Keys::Up if snake_head.dir.y == 0 => {
                        snake_head.dir = Pair { x: 0, y: -1 };
                    }
                    Keys::Down if snake_head.dir.y == 0 => {
                        snake_head.dir = Pair { x: 0, y: 1 };
                    }
                    Keys::Left if snake_head.dir.x == 0 => {
                        snake_head.dir = Pair { x: -1, y: 0 };
                    }
                    Keys::Right if snake_head.dir.x == 0 => {
                        snake_head.dir = Pair { x: 1, y: 0 };
                    }
                    _ => break,
                }
            }

            snake_head.pos.x += snake_head.dir.x;
            snake_head.pos.y += snake_head.dir.y;

            // Screen border teleportation.
            snake_head.pos.x = (snake_head.pos.x + screen.width as i32) % screen.width as i32;
            snake_head.pos.y = (snake_head.pos.y + screen.height as i32) % screen.height as i32;

            // Collition with self.
            let snake_head: Object = snake_head.clone();
            for snake_block in &snake[1..] {
                if snake_block.pos.x == snake_head.pos.x && snake_block.pos.y == snake_head.pos.y {
                    game_is_on = false;
                    break;
                }
            }

            // Collition with food.
            if food.pos.x == snake_head.pos.x && food.pos.y == snake_head.pos.y {

                // Change food coord.
                loop {
                    food.pos.x = rng.gen_range(0..screen.width) as i32;
                    food.pos.y = rng.gen_range(0..screen.height) as i32;

                    if !snake.iter().any(|snake_block| food.pos.x == snake_block.pos.x && food.pos.y == snake_block.pos.y) {
                        break;
                    }
                }

                // Update score.
                score += 1;

                // Add a new block.
                let tail: &Object = snake.last().unwrap();
                let new_pos = Pair {
                    x: tail.pos.x - tail.dir.x,
                    y: tail.pos.y - tail.dir.y,
                };

                snake.push(
                    Object {
                        pos: new_pos,
                        dir: tail.dir,
                        chr: 'X'
                    }
                );
            }

            if score == 0 {
                screen.footer = String::from(" Use arrow keys for navigation || Escape to quit. ");
            } else {
                screen.footer = format!(" Score: {} ", score);
            }


        } else {
            screen.footer = String::from(" Space to start || Escape to quit. ");
            for key in Keyboard::new() {
                match key {
                    Keys::Escape => {
                        is_running = false;
                        break;
                    },
                    Keys::Space => {
                        game_is_on = true;
                        if first_hit {
                            score = 0;
                            food.pos.x = rng.gen_range(0..screen.width) as i32;
                            food.pos.y = rng.gen_range(0..screen.height) as i32;
                            snake.clear();
                            snake.push(
                                Object {
                                    pos: Pair { x: (rng.gen_range(0..screen.width) as i32), y: (rng.gen_range(0..screen.height) as i32)},
                                    dir: Pair { x: 1, y: 0},
                                    chr: 'X'
                                }
                            );
                        } else {
                            first_hit = true;
                        }
                        break;
                    }
                    _ => break,
                }
            }
        }

        // Render.
        for snake_block in &snake {
            objects.push(snake_block.clone());
        }
        objects.push(food.clone());
        screen.render(&objects);

        // Limit FPS.
        thread::sleep(sleep_duration);
    }
}

// END
