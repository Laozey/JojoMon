#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use collections::HashMap;
use ggez::audio::*;
use ggez::Context;
use ggez::{graphics::TextFragment, GameError};
use rand::Rng;
use std::*;

#[derive(Debug)]
pub struct StandInfo {
    pub name: String,
    hp_max: i32,
    pub hp: i32,
    speed_max: i32,
    pub speed: i32,
    strength_max: i32,
    pub strength: i32,
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
            Attacks::Charisme,
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
            Attacks::MotherSoul,
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

// Ajout de string pour récup le nom de l'attaque pr le display
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Attacks {
    Zawarudo,
    Muda,
    RoadRoller,
    Charisme,
    MotherSoul,
    Ora,
    Facture,
    None,
}
impl Attacks {
    pub fn sound(&self, context: &mut Context) -> Result<Source, String> {
        match self {
            Attacks::Muda => Ok(Source::new(context, "/sound/muda_sound_effect.mp3").unwrap()),
            Attacks::RoadRoller => {
                Ok(Source::new(context, "/sound/road_roller_sound_effect.ogg").unwrap())
            }
            Attacks::Charisme => {
                Ok(Source::new(context, "/sound/charisme_sound_effect.ogg").unwrap())
            }
            Attacks::MotherSoul => {
                Ok(Source::new(context, "/sound/mothersoul_sound_effect.ogg").unwrap())
            }
            Attacks::Ora => Ok(Source::new(context, "/sound/Ora_sound_effect.mp3").unwrap()),
            Attacks::Facture => {
                Ok(Source::new(context, "/sound/facture_sound_effect.ogg").unwrap())
            }

            _ => return Err("Aucun audio".to_string()),
        }
    }
}

pub fn attack_sound_load(sounds: &mut HashMap<Attacks, Source>, context: &mut Context) {
    sounds.insert(Attacks::Ora, Attacks::Ora.sound(context).unwrap());
    sounds.insert(Attacks::Muda, Attacks::Muda.sound(context).unwrap());
    sounds.insert(
        Attacks::RoadRoller,
        Attacks::RoadRoller.sound(context).unwrap(),
    );
    sounds.insert(Attacks::Facture, Attacks::Facture.sound(context).unwrap());
    sounds.insert(
        Attacks::MotherSoul,
        Attacks::MotherSoul.sound(context).unwrap(),
    );
    sounds.insert(Attacks::Charisme, Attacks::Charisme.sound(context).unwrap());
}

pub fn basic_attack(attaquant: &mut StandInfo, receveur: &mut StandInfo, dmg: i32) {
    let total_dmg = attaquant.strength * dmg;
    receveur.hp -= total_dmg;
    println!(
        "{} inflige {} degats a {}",
        attaquant.name, total_dmg, receveur.name
    );
}

pub fn basic_heal(attaquant: &mut StandInfo, receveur: &mut StandInfo, hp_recived: i32) {
    let healing = hp_recived * attaquant.strength;
    attaquant.hp += healing;
    println!("{} ce soigne de :{}", attaquant.name, healing);
    if attaquant.hp > attaquant.hp_max {
        attaquant.hp = attaquant.hp_max;
    }
}

pub fn charisme(attaquant: &mut StandInfo, receveur: &mut StandInfo) {
    attaquant.hp += 30;
    println!("{} ce soigne de :30", attaquant.name);
    if attaquant.hp > attaquant.hp_max {
        attaquant.hp = attaquant.hp_max;
    }
}

pub fn beat_up(attaquant: &mut StandInfo, receveur: &mut StandInfo, dmg: i32) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0, 10) <= 3 {
        let degat_inflige = attaquant.strength * dmg * 2;
        println!(
            "Coup critique de {} inflige {} degats a {}",
            attaquant.name, degat_inflige, receveur.name
        );
        receveur.hp -= degat_inflige;
    } else {
        basic_attack(attaquant, receveur, dmg);
    }
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
