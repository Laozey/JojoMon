mod stand_data;
mod game_processing;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::graphics::{clear, draw, present};
use ggez::mint::Vector2;
use ggez::{Context, GameResult,ContextBuilder};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::run;

use stand_data::*;
use game_processing::*;


pub struct MyGame {
    // Your state here...
    pub turn: u32,
    pub j1_data: StandInfo,
    pub j1_attacks: Vec<Attacks>,

    pub j2_data: StandInfo,
    pub j2_attacks: Vec<Attacks>
}

impl MyGame {
    fn process(&mut self) {
        if faster_than(&self.j1_data, &self.j2_data) {
            process_attack(&mut self.j1_data, &mut self.j2_data, &mut self.j1_attacks);
            process_attack(&mut self.j2_data, &mut self.j1_data, &mut self.j2_attacks);

        } else {
            process_attack(&mut self.j2_data, &mut self.j1_data, &mut self.j2_attacks);
            process_attack(&mut self.j1_data, &mut self.j2_data, &mut self.j1_attacks);
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn key_up_event(&mut self, _context: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if (self.turn % 2) == 0 {
            // Player 1 turn ...
            match keycode {
                k @ KeyCode::A | k @ KeyCode::Z | k @ KeyCode::E | k @ KeyCode::R => {
                    self.j1_attacks.push(select_attack(&self.j1_data,k));
                    println!("You pressed the {:?} Key !", &k);
                    self.turn += 1;
                }
                _ => (),
            }
        } else {
            // Player 2 turn ...
            match keycode {
                k @ KeyCode::A | k @ KeyCode::Z | k @ KeyCode::E | k @ KeyCode::R => {
                    self.j2_attacks.push(select_attack(&self.j2_data,k));
                    println!("You pressed the {:?} Key !", &k);
                    self.turn += 1;
                    self.process();
                }
                _ => (),
            }
        }
    }

    // TODO Ajouter le text des attaques plus l'input associé
    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        clear(context, Color::from_rgb(120, 120, 120));
        // Draw code here...
        let mut meshes: Vec<Mesh> = Vec::new();

        // Button rect, color and offsets
        let mut rect = Rect::new(25.0, 400.0, 100.0, 75.0);
        let mut offset = Vector2 { x: 0.0, y: 0.0 };
        let mut rcolor = Color::from_rgb(255, 240, 100);
        let mut rcolor_switch: (u8, u8, u8) = (255, 240, 100);
        
        // Create a mesh then move the offset (UI)
        for i in 0..=7 {
            match i {
                0 | 2 => offset.x = 120.0,
                1 => {
                    offset.x = 410.0;
                    rcolor_switch = (255, 100, 100);
                }
                3 => {
                    offset.x = 0.0;
                    offset.y = 95.0;
                }
                4 | 6 => {
                    offset.x = -120.0;
                    offset.y = 0.0;
                }
                5 => {
                    offset.x = -410.0;
                    rcolor_switch = (255, 240, 100);
                }
                _ => (),
            };
            meshes.push(
                Mesh::new_rectangle(context, DrawMode::stroke(5.0), rect, rcolor)
                    .expect("Couldn't inisialise Mesh"),
            );
            // Adjustements...
            rcolor = Color::from_rgb(rcolor_switch.0, rcolor_switch.1, rcolor_switch.2);
            rect.translate(offset)
        }
        // Draw meshes
        for meshe in meshes {
            draw(context, &meshe, DrawParam::default())?;
        }
        present(context)?;
        Ok(())
    }
}


fn main() -> GameResult {
    // Make a Context
    let window_setup = WindowSetup::default()
        .title("Jojomon, Gotta Ora Ora Ora'em all!")
        .vsync(false);

    let (mut context, mut event_loop) = match ContextBuilder::new("Jojomon", "Quentin Epron")
        .window_mode(WindowMode::default())
        .window_setup(window_setup)
        .build()
    {
        Ok(ctxbuilder) => ctxbuilder,
        Err(error) => panic!("Couldn't create ggez context : {}", error),
    };

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame {
        turn: 0,
        j1_data: StandInfo::dio(),
        j2_data: StandInfo::jotaro(),
        j1_attacks: Vec::new(),
        j2_attacks: Vec::new(),
    };
    // Run!
    Ok(match run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(error) => println!("Error occured: {}", error),
    })
}
