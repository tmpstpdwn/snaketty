
// IMPORTS

use std::io::Write;
use std::{thread, time};
use k_board::{keyboard::Keyboard, keys::Keys};
use rand::Rng;

// Pair Struct

struct Pair {
    x: i32,
    y: i32,
} 

impl Pair {
    fn clone(&self) -> Self {
        Pair {
            x: self.x,
            y: self.y,
        }
    }
}

// Object Struct

struct Object {
    pos: Pair,
    dir: Pair,
    chr: char,
}

impl Object {
    fn clone(&self) -> Self {
        Object {
            pos: self.pos.clone(),
            dir: self.dir.clone(),
            chr: self.chr,
        }
    }
}

// Clear screen fn.

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
}

// Render fn.

fn render(width: &u32, height: &u32, objects: &Vec<Object>, score: &u32) {
    clear_screen();

    println!("\nSNAKE-GAME\n----------");

    // Header.
    for i in 0..*width + 2 {
        if i == 0 {
            print!("┌");
        } else if i == *width + 1 {
            print!("┐");
        } else {
            print!("─");
        }
    }
    println!();

    // Screen.
    for y in 0..*height {
        for x in 0..*width {
            let mut printed = false;

            if x == 0 {
                print!("│");
            } 

            for obj in objects {
                if obj.pos.x == x as i32 && obj.pos.y == y as i32 {
                    print!("{}", obj.chr);
                    printed = true;
                    break;
                }
            }

            if !printed {
                print!(" ");
            }

            if x == *width - 1 {
                print!("│");
            }
        }
        println!();
    }

    // Footer.
    for i in 0..*width + 2 {
        if i == 0 {
            print!("└");
        } else if i == *width + 1 {
            print!("┘");
        } else {
            print!("─");
        }
    }
    println!();
    println!("score: {}", *score);
}

// Main fn.

fn main() {

    let width: u32 = 100;
    let height: u32 = 25;

    let fps: u64 = 60;
    let sleep_duration: time::Duration = time::Duration::from_millis(1000 / fps); 
    let mut rng = rand::thread_rng();

    let mut score: u32 = 0;

    let mut snake: Vec<Object> = vec![
        Object {
            pos: Pair { x: (rng.gen_range(0..=width) as i32), y: (rng.gen_range(0..=height) as i32)},
            dir: Pair { x: 1, y: 0},
            chr: 'X'
        }
    ]; 

    let mut food: Object = Object {
        pos: Pair { x: (rng.gen_range(0..=width) as i32), y: (rng.gen_range(0..=height) as i32)},
        dir: Pair { x: 1, y: 0},
        chr: 'O'
    };

    loop {

        // Objects to be rendered.
        let mut objects: Vec<Object> = vec![];

        // Body movement.
        let mut index = snake.len() - 1;
        while index > 0 {
            snake[index].pos = snake[index - 1].pos.clone();
            snake[index].dir = snake[index - 1].dir.clone();
            index -= 1;
        }

        // Head movement.
        let snake_head: &mut Object = &mut snake[0];

        for key in Keyboard::new() {
            match key {
                Keys::Up => {
                    snake_head.dir.x =  0;
                    snake_head.dir.y = -1;
                },
                Keys::Down => {
                    snake_head.dir.x =  0;
                    snake_head.dir.y =  1;
                },
                Keys::Left => {
                    snake_head.dir.x = -1;
                    snake_head.dir.y =  0;
                },
                Keys::Right => {
                    snake_head.dir.x =  1;
                    snake_head.dir.y =  0;
                },
                _ => break
            }
        }

        snake_head.pos.x += snake_head.dir.x;
        snake_head.pos.y += snake_head.dir.y;

        // Screen border teleportation.
        if snake_head.pos.x > width as i32 - 1 {
            snake_head.pos.x = 0;
        } else if snake_head.pos.x < 0 {
            snake_head.pos.x = width as i32 - 1;
        }

        if snake_head.pos.y > height as i32 - 1 {
            snake_head.pos.y = 0;
        } else if snake_head.pos.y < 0 {
            snake_head.pos.y = height as i32 - 1;
        }

        // Collition with food.
        if food.pos.x == snake_head.pos.x && food.pos.y == snake_head.pos.y {

            // Change food coord.
            food.pos.x = rng.gen_range(0..=width) as i32;
            food.pos.y = rng.gen_range(0..=height) as i32;

            // Update score.
            score += 1;

            // Add a new block.
            let tail: &Object = snake.last().unwrap();
            let mut x: i32 = 0;
            let mut y: i32 = 0;

            if tail.dir.x > 0 {
                x = tail.pos.x - 1;
                y = tail.pos.y;
            } else if tail.dir.x < 0 {
                x = tail.pos.x + 1;
                y = tail.pos.y;
            }

            if tail.dir.y > 0 {
                x = tail.pos.x;
                y = tail.pos.y - 1;
            } else if tail.dir.y < 0 {
                x = tail.pos.x;
                y = tail.pos.y + 1;
            }

            snake.push(
                Object {
                    pos: Pair { x: x, y: y },
                    dir: tail.dir.clone(),
                    chr: 'X'
                }
            );
        }

        // Render.
        for snake_block in &snake {
            objects.push(snake_block.clone());
        }
        objects.push(food.clone());
        render(&width, &height, &objects, &score);

        // Limit FPS.
        thread::sleep(sleep_duration);
    }
}

// END
