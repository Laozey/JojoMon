use crate::stand_data::*;
use ggez::audio::*;
use ggez::event::KeyCode;
use std::collections::HashMap;
use std::process;
// Ajout de string pour récup le nom de l'attaque pr le display
pub fn process_attack(
    j1: &mut StandInfo,
    j2: &mut StandInfo,
    attack_to_process: &mut Vec<Attacks>,
    sounds: &mut HashMap<Attacks, Source>,
) {
    for attack in attack_to_process.iter_mut() {
        match attack {
            Attacks::Facture => basic_attack(j1, j2, 4),
            Attacks::Ora => beat_up(j1, j2, 2),
            Attacks::Muda => beat_up(j1, j2, 2),
            Attacks::RoadRoller => basic_attack(j1, j2, 4),
            Attacks::Charisme => charisme(j1, j2),
            Attacks::MotherSoul => mother_soul(j1, j2),
            _ => (),
        }
        let sound_to_play = sounds.get_mut(attack).unwrap();
        sound_to_play.play().unwrap();
        while sound_to_play.playing() {}
    }
    *attack_to_process = Vec::new();
    test_end_game(&j1, &j2);
}

pub fn select_attack(stand: &StandInfo, keycode: KeyCode) -> Attacks {
    let current_attack;
    println!("{:?}", stand);
    println!("Veuillez choisir une attaque:");
    match keycode {
        KeyCode::A => current_attack = stand.attack1,
        KeyCode::Z => current_attack = stand.attack2,
        KeyCode::E => current_attack = stand.attack3,
        KeyCode::R => current_attack = stand.attack4,
        _ => panic!("Invalid Key"),
    };
    return current_attack;
}

fn test_end_game(j1: &StandInfo, j2: &StandInfo) {
    if j1.hp <= 0 {
        println!("{} has Won", j2.name);
        process::exit(0);
    } else if j2.hp <= 0 {
        println!("{} has Won", j1.name);
        process::exit(0);
    }
}
