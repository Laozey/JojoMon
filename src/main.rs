#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use ggez::*;
use legion::*;

mod stand_data;
use stand_data::*;

//*Player 1 select attack
//*Player 2 select attack
//* Process Effect and Attack Order based on Player speed
//* Go next turn

fn main() {
    let mut world = World::default();
    let dio = StandStats::dio();
    let jotaro = StandStats::jotaro();
   
    let perso1 = world.push((jotaro,));
    let perso2 = world.push((dio,));
    
}
