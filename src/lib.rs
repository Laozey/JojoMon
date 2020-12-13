use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{Context, GameResult, graphics};
use ggez::graphics::Color; 

pub struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_context: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, Color::from_rgb(120, 120, 120));
        // Draw code here...
        graphics::present(context)
    }
}