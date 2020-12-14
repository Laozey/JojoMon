mod stand_data;

use std::cmp::min;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{clear, draw, present};
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::mint::Vector2;
use ggez::{Context, GameResult};
use legion::World;
use stand_data::StandStats;

pub struct MyGame {
    // Your state here...
    world: World,
    turn: u32,
}

impl MyGame {
    pub fn new(_context: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut w = World::default();
        let dio = StandStats::dio();
        let jotaro = StandStats::jotaro();
        let perso1 = w.push((jotaro,));
        let perso2 = w.push((dio,));
        MyGame {
            // ...
            world: w,
            turn: 0,
        }
    }

    fn process() {
        // Process all events separetely from the event handler
        println!("Ora to Muda")
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
                k @ KeyCode::A | k @ KeyCode::Z | k @ KeyCode::Q | k @ KeyCode::S => {
                    println!("You pressed the {:?} Key !", &k);
                    self.turn += 1;
                }
                _ => (),
            }
        } else {
            // Player 2 turn ...
            match keycode {
                k @ KeyCode::O | k @ KeyCode::P | k @ KeyCode::L | k @ KeyCode::M => {
                    println!("You pressed the {:?} Key !", &k);
                    self.turn += 1;
                    MyGame::process();
                }
                _ => (),
            }
        }
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if (self.turn % 2) == 0 {
            // Player 1 turn ...
            match keycode {
                k @ KeyCode::A | k @ KeyCode::Z | k @ KeyCode::Q | k @ KeyCode::S => {
                    println!("You pressed the {:?} Key !", &k);
                    self.turn += 1;
                }
                _ => (),
            }
        } else {
            // Player 2 turn ...
            match keycode {
                k @ KeyCode::O | k @ KeyCode::P | k @ KeyCode::L | k @ KeyCode::M => {
                    println!("You pressed the {:?} Key !", &k);
                    self.turn += 1;
                    MyGame::process();
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
