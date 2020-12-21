#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use rand::Rng;
use std::*;

#[derive(Debug)]
pub struct StandInfo {
    pub name: String,
    hp_max: i32,
    pub hp: i32,
    speed_max: i32,
    speed: i32,
    strength_max: i32,
    strength: i32,
    pub attack1: Attacks,
    pub attack2: Attacks,
    pub attack3: Attacks,
    pub attack4: Attacks,
}
impl StandInfo {
    pub fn new(
        name: String,
        hp: i32,
        speed: i32,
        strength: i32,
        attack1: Attacks,
        attack2: Attacks,
        attack3: Attacks,
        attack4: Attacks,
    ) -> StandInfo {
        StandInfo {
            name,
            hp_max: hp,
            hp,
            speed_max: speed,
            speed,
            strength,
            strength_max: strength,
            attack1,
            attack2,
            attack3,
            attack4,
        }
    }

    pub fn dio() -> StandInfo {
        return StandInfo::new(
            "Dio".to_string(),
            200,
            15,
            10,
            Attacks::Muda,
            Attacks::RoadRoller,
            Attacks::Charisme(2),
            Attacks::Zawarudo,
        );
    }
    pub fn jotaro() -> StandInfo {
        return StandInfo::new(
            "Jotaro".to_string(),
            200,
            15,
            10,
            Attacks::Ora,
            Attacks::Facture,
            Attacks::MotherSoul(2),
            Attacks::Zawarudo,
        );
    }
}

pub fn faster_than(stand1: &StandInfo, stand2: &StandInfo) -> bool {
    if stand1.speed > stand2.speed {
        return true;
    }
    return false;
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Attacks {
    Zawarudo,
    Muda,
    RoadRoller,
    Charisme(i32),
    MotherSoul(i32),
    Ora,
    Facture,
    None,
}

pub fn basic_attack(attaquant: &mut StandInfo, receveur: &mut StandInfo, dmg: i32) {
    receveur.hp -= attaquant.strength * dmg;
}

pub fn charisme(attaquant: &mut StandInfo, receveur: &mut StandInfo) {
    //* est la force du soin de charisme
    attaquant.hp += 30;
    println!("{} ce soigne de :30", attaquant.name);
    if attaquant.hp > attaquant.hp_max {
        attaquant.hp = attaquant.hp_max;
    }
}
pub fn beat_up(attaquant: &mut StandInfo, receveur: &mut StandInfo, dmg: i32) {
    let mut rng = rand::thread_rng();
    let mut degat_inflige = attaquant.strength * dmg;
    if rng.gen_range(0, 10) <= 3 {
        degat_inflige *= 2;
        println!(
            "Coup critique de {} inflige {} degats a {}",
            attaquant.name, degat_inflige, receveur.name
        );
    } else {
        println!(
            "{} inflige {} degats a {}",
            attaquant.name, degat_inflige, receveur.name
        );
    }
    receveur.hp -= degat_inflige;
}

pub fn mother_soul(attaquant: &mut StandInfo, receveur: &mut StandInfo) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0, 10) == 1 {
        receveur.hp = 0;
        println!(
            "{} Inflige un coup critique a {} il meurt sous la pression",
            attaquant.name, receveur.name
        );
    } else {
        //TODO Retirer des stats
    }
}
pub fn long_effect_attack(attaque_to_test: &Attacks) -> bool {
    match attaque_to_test {
        Attacks::Charisme(duration) => {
            if *duration == 0 {
                return false;
            } else {
                return true;
            }
        }
        Attacks::MotherSoul(duration) => {
            if *duration == 0 {
                return false;
            } else {
                return true;
            }
        }
        _ => return false,
    }
}
