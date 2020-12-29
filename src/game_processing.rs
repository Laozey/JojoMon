use crate::stand_data::*;
use ggez::audio::*;
use ggez::event::KeyCode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::process;

// Ajout de string pour récup le nom de l'attaque pr le display
///Permet d'effectuer le tour d'un joueur effet et attaque comprise
pub fn process_turn(
    j1: &mut StandInfo,
    j2: &mut StandInfo,
    attack_to_process: &mut Vec<Attacks>,
    sounds: &mut HashMap<Attacks, Source>,
) {
    j1.reset_stand_info();
    let done_effect = j1.status.to_vec();
    let done_effect = done_effect.into_iter().collect::<HashSet<_>>();
    for effect in done_effect.iter() {
        effect_func(j1, &effect, attack_to_process);
        let remov_pos = j1.status.iter().position(|t| effect == t);
        match remov_pos {
            Some(x) => {
                j1.status.remove(x);
            }
            _ => (),
        };
    }

    for attack in attack_to_process.iter_mut() {
        match attack {
            Attacks::Zawarudo(_) => zawarudo(j1, j2),

            Attacks::Facture => basic_attack(j1, j2, 4),
            Attacks::Ora => beat_up(j1, j2, 2),
            Attacks::Muda => beat_up(j1, j2, 2),
            Attacks::RoadRoller => basic_attack(j1, j2, 4),
            Attacks::Charisme => charisme(j1, j2),
            Attacks::MotherSoul => mother_soul(j1, j2),
            Attacks::CrossFire => cross_fire(j1, j2),
            Attacks::RedBind => red_bind(j1, j2),
            Attacks::FireBall => basic_attack(j1, j2, 4),

            Attacks::Rafale => beat_up(j1, j2, 4),
            Attacks::SwordShot => sword_shot(j1, j2),
            Attacks::ArmorDrop => armor_drop(j1, j2),

            Attacks::EmeraldSplash => basic_attack(j1, j2, 4),
            Attacks::Ligotage => ligotage(j1, j2),
            Attacks::MineField => mine_field(j1, j2),
            _ => (),
        }
        let sound_to_play = sounds.get_mut(attack).unwrap(); // Gérer None

        sound_to_play.play().unwrap();
        while sound_to_play.playing() {}
    }
    *attack_to_process = Vec::new();
    test_end_game(&j1, &j2);
}
/// Permet la gestion du choix d'attaque
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
/// Quitter si un des joueurs est mort
fn test_end_game(j1: &StandInfo, j2: &StandInfo) {
    if j1.hp <= 0 {
        println!("{} has Won", j2.name);
        process::exit(0);
    } else if j2.hp <= 0 {
        println!("{} has Won", j1.name);
        process::exit(0);
    }
}
