// Copyright (C) 2025 github.com/tmpstpdwn
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// IMPORTS.
use std::io::Write;
use std::{thread, time};
use k_board::{keyboard::Keyboard, keys::Keys};
use termion::{terminal_size, cursor};
use rand::Rng;

// Consts.
const FPS: u64 = 30;
const SNAKE_BLOCK: char = '■';
const FOOD_BLOCK: char = '●';
const SCREEN_DIAMENTION_PERC: f32 = 0.7;
const MIN_WIDTH: u32 = 50;
const MIN_HEIGHT: u32 = 15;

// Pair Struct.
#[derive(Clone, Copy)]
struct Pair {
    x: i32,
    y: i32,
} 

// Object Struct.
struct Object {
    pos: Pair,
    dir: Pair,
    chr: char,
}

// Screen Struct.
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
    fn new() -> Option<Self> {

        // Get terminal size using termion.
        let (term_width, term_height) = match terminal_size() {
            Ok(size) => size,
            Err(_) => return None,
        };

        // Calculate % of terminal width and height.
        let width = (term_width as f32 * SCREEN_DIAMENTION_PERC) as u32;
        let height = (term_height as f32 * SCREEN_DIAMENTION_PERC) as u32;

        if width < MIN_WIDTH || height < MIN_HEIGHT {
            return None
        }

        // Calculate padding to center the screen.
        let padx = (term_width as u32 - (width + 2)) / 2;
        let pady = (term_height as u32 - (height + 2)) / 2;

        // Create and return a new Screen instance.
        Some (
            Screen {
                width,
                height,
                padx,
                pady,
                header: String::new(),
                footer: String::new(),
            }
        )
        
    }

    // Clear screen fn.
    fn clear_screen() {
        print!("\x1B[2J\x1B[H");
    }

    // Render fn.
    fn render(&self, objects: &[&Object]) {

        // O/P String.
        let mut output: String = String::new();

        // pady.
        output.push_str(&"\n".repeat(self.pady as usize));

        // Header.
        output.push_str(&" ".repeat(self.padx as usize)); // Padx.
        output.push_str("┌"); 
        if self.header.len() < self.width as usize{
            let header_padding = (self.width as usize - self.header.len()) / 2;
            let remaining_padding = self.width as usize - (header_padding + self.header.len());
            output.push_str(&"─".repeat(header_padding));
            output.push_str(&self.header);
            output.push_str(&"─".repeat(remaining_padding));
        } else {
            output.push_str(&"─".repeat(self.width as usize));
        }
        output.push_str("┐\n");

        // Screen.
        for y in 0..self.height {
            output.push_str(&" ".repeat(self.padx as usize)); // Padx.
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
        output.push_str(&" ".repeat(self.padx as usize)); // Padx.
        output.push_str("└");
        if self.footer.len() < self.width as usize{
            let footer_padding = (self.width as usize - self.footer.len()) / 2;
            let remaining_padding = self.width as usize - (footer_padding + self.footer.len());
            output.push_str(&"─".repeat(footer_padding));
            output.push_str(&self.footer);
            output.push_str(&"─".repeat(remaining_padding));
        } else {
            output.push_str(&"─".repeat(self.width as usize));
        }
        output.push_str("┘\n");


        // Pady.
        output.push_str(&"\n".repeat(self.pady as usize));

        // Render.
        Screen::clear_screen();
        print!("{}", output);
        std::io::stdout().flush().unwrap();
    }
}

// Gamesate enum.
enum GameState {
    Playing,
    GameOver,
}

// Main fn.
fn main() {
    print!("{}", cursor::Hide);

    let mut screen = Screen::new().unwrap_or_else(|| {
        eprintln!("Failed to create screen: terminal size too small");
        std::process::exit(1);
    });

    screen.header = String::from(" SNAKE-TTY ");

    let sleep_duration: time::Duration = time::Duration::from_millis(1000 / FPS); 
    let mut rng = rand::thread_rng();

    let mut score: u32 = 0;

    let mut snake: Vec<Object> = vec![
        Object {
            pos: Pair { x: (rng.gen_range(0..screen.width) as i32), y: (rng.gen_range(0..screen.height) as i32)},
            dir: Pair { x: 1, y: 0},
            chr: SNAKE_BLOCK
        }
    ]; 

    let mut food: Object = Object {
        pos: Pair { x: (rng.gen_range(0..screen.width) as i32), y: (rng.gen_range(0..screen.height) as i32)},
        dir: Pair { x: 1, y: 0},
        chr: FOOD_BLOCK
    };

    let mut first_hit: bool = true;
    let mut is_running: bool = true;
    let mut gamestate: GameState = GameState::GameOver;

    while is_running {

        if let GameState::Playing = gamestate {

            // Body movement.
            let mut index = snake.len() - 1;
            while index > 0 {
                snake[index].pos = snake[index - 1].pos;
                snake[index].dir = snake[index - 1].dir;
                index -= 1;
            }

            // Head movement.
            for key in Keyboard::new() {
                match key { Keys::Escape => is_running = false,
                    Keys::Up if snake[0].dir.y == 0 => snake[0].dir = Pair { x: 0, y: -1 },
                    Keys::Down if snake[0].dir.y == 0 => snake[0].dir = Pair { x: 0, y: 1 },
                    Keys::Left if snake[0].dir.x == 0 => snake[0].dir = Pair { x: -1, y: 0 },
                    Keys::Right if snake[0].dir.x == 0 => snake[0].dir = Pair { x: 1, y: 0 },
                    _ => break,
                }
            }

            snake[0].pos.x += snake[0].dir.x;
            snake[0].pos.y += snake[0].dir.y;

            // Screen border teleportation.
            snake[0].pos.x = (snake[0].pos.x + screen.width as i32) % screen.width as i32;
            snake[0].pos.y = (snake[0].pos.y + screen.height as i32) % screen.height as i32;

            // Collition with food.
            if food.pos.x == snake[0].pos.x && food.pos.y == snake[0].pos.y {

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
                        chr: SNAKE_BLOCK
                    }
                );
            }

            // Collition with self.
            for snake_block in &snake[1..] {
                if snake_block.pos.x == snake[0].pos.x && snake_block.pos.y == snake[0].pos.y {
                    gamestate = GameState::GameOver;
                    break;
                }
            }

            if score == 0 {
                screen.footer = String::from(" Use arrow keys for navigation || Escape to quit. ");
            } else {
                screen.footer = format!(" Score: {} ", score);
            }


        } else {
            for key in Keyboard::new() {
                match key {
                    Keys::Escape => is_running = false,
                    Keys::Space => {
                        gamestate = GameState::Playing;
                        if !first_hit {
                            score = 0;
                            food.pos.x = rng.gen_range(0..screen.width) as i32;
                            food.pos.y = rng.gen_range(0..screen.height) as i32;
                            snake.clear();
                            snake.push(
                                Object {
                                    pos: Pair { 
                                        x: (rng.gen_range(0..screen.width) as i32), 
                                        y: (rng.gen_range(0..screen.height) as i32)
                                    },
                                    dir: Pair { x: 1, y: 0},
                                    chr: SNAKE_BLOCK
                                }
                            );
                        } else {
                            first_hit = false;
                        }
                    }
                    _ => break,
                }
            }
            screen.footer = String::from(" Space to start || Escape to quit. ");
        }

        // Render.
        let mut objects: Vec<&Object> = Vec::with_capacity(snake.len() + 1);
        objects.extend(snake.iter());
        objects.push(&food);
        screen.render(&objects);

        // Limit FPS.
        thread::sleep(sleep_duration);
    }
    print!("{}", cursor::Show);
}

// END
