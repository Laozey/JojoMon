//*Pour Ma Santé mentale*/
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use collections::HashMap;
use ggez::{GameResult, audio::*};
use ggez::Context;
use rand::Rng;
use std::*;
use ggez::nalgebra as na;
use na::*;

//TODO Ajouter Quelques Effet/Debuf ?
//TODO Ajouter 4 autres Stand et leurs attaque

//* Stats des Stands */
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
    pub status: Vec<Status>,
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
            status: Vec::new(),
        }
    }
    pub fn reset_stand_info(&mut self){
        self.speed = self.speed_max;
        self.strength = self.strength_max;
    }


    //Un Personage
    pub fn dio() -> StandInfo {
        return StandInfo::new(
            "Dio".to_string(),
            200,
            15,
            10,
            Attacks::Muda,
            Attacks::RoadRoller,
            Attacks::Charisme,
            Attacks::Zawarudo(1),
        );
    }
    //Un Personage
    pub fn jotaro() -> StandInfo {
        return StandInfo::new(
            "Jotaro".to_string(),
            200,
            15,
            10,
            Attacks::Ora,
            Attacks::Facture,
            Attacks::MotherSoul,
            Attacks::Zawarudo(2),
        );
    }

    pub fn display_player_pp(&self, context: &mut Context) -> GameResult<()> {
        let mut image = Image::new(context, "/HEYimHeroic_3DS_FACE-024_Matt-Wii.png")?;
        
        match self.name.as_str() {
            "Dio" => image = Image::new(context, "/Eoh_DIO.png")?,
            "Jotaro" => image = Image::new(context, "/Jotaro_SC_Infobox_Manga.png")?,
            _ => (),
        }
        
        graphics::draw(
            context,
            &image,
            DrawParam::default()
                .dest(na::Point2::new(130.0, 100.0))
                .scale(na::Vector2::new(0.5, 0.5)),
        )?;

        Ok(())

    }
}

pub fn faster_than(stand1: &StandInfo, stand2: &StandInfo) -> bool {
    if stand1.speed > stand2.speed {
        return true;
    }
    return false;
}

//* Les Attaques disponible dans le jeu */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Attacks {
    Zawarudo(i32),
    Muda,
    RoadRoller,
    Charisme,
    MotherSoul,
    Ora,
    Facture,
    None,
}
impl Attacks {
    //* Les sons pour chaque attaque */
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
            Attacks::Zawarudo(1) => Ok(Source::new(context, "/sound/Za_Warudo_Dio.mp3").unwrap()),
            Attacks::Zawarudo(2) => {
                Ok(Source::new(context, "/sound/Za_Warudo_Jotaro.mp3").unwrap())
            }

            _ => return Err("Aucun audio".to_string()),
        }
    }
}
//*Permet de charger tout les sons dans le jeu */
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
    sounds.insert(
        Attacks::Zawarudo(1),
        Attacks::Zawarudo(1).sound(context).unwrap(),
    );
    sounds.insert(
        Attacks::Zawarudo(2),
        Attacks::Zawarudo(2).sound(context).unwrap(),
    );
}
//* Une simple attaque */
pub fn basic_attack(attaquant: &mut StandInfo, receveur: &mut StandInfo, dmg: i32) {
    let total_dmg = attaquant.strength * dmg;
    receveur.hp -= total_dmg;
    println!(
        "{} inflige {} degats a {}",
        attaquant.name, total_dmg, receveur.name
    );
}
//* Un simple soin */
pub fn basic_heal(attaquant: &mut StandInfo, _: &mut StandInfo, hp_recived: i32) {
    let healing = hp_recived * attaquant.strength;
    attaquant.hp += healing;
    println!("{} ce soigne de :{}", attaquant.name, healing);
    if attaquant.hp > attaquant.hp_max {
        attaquant.hp = attaquant.hp_max;
    }
}

pub fn charisme(attaquant: &mut StandInfo, receveur: &mut StandInfo) {
    basic_heal(attaquant, receveur, 3);
    for i in 0..=4 {
        attaquant.status.push(Status::Regeneration)
    }
}
//* Basic Attaque avec possibliter de critique */
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
//* Une Attaque Speciale */
pub fn mother_soul(attaquant: &mut StandInfo, receveur: &mut StandInfo) {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0, 10) == 1 {
        receveur.hp = 0;
        println!(
            "{} Inflige un coup critique a {} il meurt sous la pression",
            attaquant.name, receveur.name
        );
    } else {
        receveur.status.push(Status::SpeedLost);
        receveur.status.push(Status::StrengthLost);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Status {
    Regeneration,
    DmgSec,
    Etourdi,
    SpeedLost,
    StrengthLost,
}

pub fn effect_func(stand: &mut StandInfo, effect: &Status,attack_to_process: &mut Vec<Attacks>,) {
    match effect {
        Status::Regeneration => {
            stand.hp += 10;
            if stand.hp >= stand.hp_max {
                stand.hp = stand.hp_max;
            }
        }
        Status::DmgSec=> stand.hp -=10,
        Status::Etourdi=> *attack_to_process = Vec::new(),
        Status::SpeedLost=> stand.speed -= 10,
        Status::StrengthLost=> stand.strength -= 3,
    }
}
