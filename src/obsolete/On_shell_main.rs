#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use ggez::*;
use std::io;
mod stand_data;
use stand_data::*;
use std::process;

//*Player 1 select attack
//*Player 2 select attack
//* Process Effect and Attack Order based on Player speed
//* Go next turn
//* Having fun adding attacks and Stands
fn game_start(joueur_1: &mut StandInfo,joueur_2: &mut StandInfo){
    let mut joueur_1_selected_attack: Vec<Attacks> = Vec::new();
    let mut joueur_2_selected_attack: Vec<Attacks> = Vec::new();

   loop{
    //* Dio lance Za Warudo = il lance 2 attack supplementaire sauf si Jotaro alors 1 attack
    //* Jotaro lance Za Warudo en même temps que Dio alors rien ne se passe
    //* Jotaro lance Za Warudo = il lance 1 attack supplementaire sauf si Dio alors rien ne se passe
        joueur_1_selected_attack.push(select_attack(&joueur_1));
        joueur_2_selected_attack.push(select_attack(&joueur_2));

        if faster_than(&joueur_1, &joueur_2) {
            process_attack(joueur_1, joueur_2, &mut joueur_1_selected_attack);
            process_attack(joueur_2, joueur_1, &mut joueur_2_selected_attack);

        } else {
            process_attack(joueur_2, joueur_1, &mut joueur_2_selected_attack);
            process_attack(joueur_1, joueur_2, &mut joueur_1_selected_attack);
        }
}
}

pub fn select_attack(stand: &StandInfo) -> Attacks {
    let current_attack;
    println!("{:?}",stand);
    println!("Veuillez choisir une attaque entre 1 et 4 :");
    let mut buffer = String::new();
    let flux = io::stdin();
    flux.read_line(&mut buffer)
        .expect("Erreur lors de la lecture du terminal");
    let buffer = buffer
        .trim()
        .parse::<i32>()
        .expect("Erreur lors du parsing");
    match buffer {
        1 => current_attack = stand.attack1,
        2 => current_attack = stand.attack2,
        3 => current_attack = stand.attack3,
        4 => current_attack = stand.attack4,
        _ => current_attack = stand.attack1,
    };
    return current_attack;
}

fn process_attack(j1: &mut StandInfo, j2: &mut StandInfo, attack_to_process: &mut Vec<Attacks>) {
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

fn main() {
    let mut joueur_1 = StandInfo::dio();
    let mut joueur_2 = StandInfo::jotaro();
    game_start(&mut joueur_1, &mut joueur_2)
}
