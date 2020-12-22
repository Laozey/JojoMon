use crate::stand_data::*;
use std::process;
use ggez::event::KeyCode;

// Ajout de string pour récup le nom de l'attaque pr le display
pub fn process_attack(j1: &mut StandInfo, j2: &mut StandInfo, attack_to_process: &mut Vec<Attacks>) {
    for attack in attack_to_process.iter_mut() {
        match attack {
            Attacks::Facture => basic_attack(j1, j2, 4),
            Attacks::Ora => beat_up(j1, j2, 2),
            Attacks::Muda => beat_up(j1, j2, 2),
            Attacks::RoadRoller => basic_attack(j1, j2, 4),
            Attacks::Charisme(duration) => {
                *duration -= 1;
                charisme(j1, j2);
            }
            Attacks::MotherSoul(duration) => {
                *duration -= 1;
                mother_soul(j1, j2);
            }
            _ =>(),
        }
        if long_effect_attack(attack) == false{
            *attack = Attacks::None;
        }
    }

    attack_to_process.retain(|x| *x != Attacks::None);

    test_end_game(&j1, &j2);

}


pub fn select_attack(stand: &StandInfo,keycode: KeyCode) -> Attacks {
    let current_attack;
    println!("{:?}",stand);
    println!("Veuillez choisir une attaque:");
    match keycode {
        KeyCode::A => current_attack = stand.attack1,
        KeyCode::Z => current_attack = stand.attack2,
        KeyCode::E => current_attack = stand.attack3,
        KeyCode::R => current_attack = stand.attack4,
        _ => panic!("Invalid Key")
    };
    return current_attack;
}

fn test_end_game(j1 :&StandInfo,j2 :&StandInfo){
    if j1.hp <= 0{
        println!("{} has Won",j2.name);
        process::exit(0);
    }
    else if j2.hp <= 0{
        println!("{} has Won",j1.name);
        process::exit(0);
    }
}
