#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod stand_data;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::run;
use ggez::{ContextBuilder, GameResult};
use legion::*;
use stand_data::*;
use jojomon::MyGame;

//*Player 1 select attack
//*Player 2 select attack
//* Process Effect and Attack Order based on Player speed
//* Go next turn

fn main() -> GameResult {
    let mut world = World::default();
    let dio = StandStats::dio();
    let jotaro = StandStats::jotaro();
    let perso1 = world.push((jotaro,));
    let perso2 = world.push((dio,));

    // Make a Context.
    let window_mode = WindowMode::default().maximized(true).resizable(true);

    let window_setup = WindowSetup::default()
        .title("Jojomon, Gotta Ora Ora Ora'em all!")
        .vsync(false);

    let (mut context, mut event_loop) = match ContextBuilder::new("Jojomon", "Quentin Epron")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()
    {
        Ok(ctxbuilder) => ctxbuilder,
        Err(error) => panic!("Couldn't create ggez context : {}", error),
    };

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut context);

    // Run!
    Ok(match run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(error) => println!("Error occured: {}", error),
    })
}
