#[derive(Debug)]
pub enum Effets {
    Stun(u32),
    Regeneration(u32, u32),
    LostAttack(u32, u32),
    LostSpeed(u32, u32),
    Lostprecision(u32, u32),
    Lostdodge(u32, u32),
}

//? A voir si on l'ajoute
pub enum Passifs {
    //* Dio comes back to life
    UnDead,

    //* La puissance Jotaro de augemente a mesure qu'il s'approche de son adversaire
    //* "+ de tour = + de stats"
    BigBalls,
}

pub enum Attacks {
    //? Commun capacity
    //* Stun l'adversaire pendant x tour
    Zawarudo,
    //* Une attaque special "déluge de coups"
    BeatUp(u32),

    //? Dio capacity
    Muda(u32),
    RoadRoller(u32),
    Charisme, //* Dio tape une pose
    //* Dio est beau donc toute attaque est annulé

    //? Jotaro capacity
    //* Jotaro jure sur la tête de sa mére
    //* Met le doute dans le coeur de son adversaire il perds bcp de precision pour 2 tour
    //* Dans certain cas l'adversaire meurt
    MotherSoul,
    Ora(u32),
}

#[derive(Debug)]
pub struct StandStats {
    hp: i32,
    speed: u32,
    attack: u32,
    accuracy: u32,
    dodge: u32,
    status: Vec<Effets>,
}

impl StandStats {
    fn new(hp: i32, speed: u32, attack: u32, accuracy: u32, dodge: u32) -> Self {
        StandStats {
            hp: hp,
            speed: speed,
            attack: attack,
            accuracy: accuracy,
            dodge: dodge,
            status: Vec::new(),
        }
    }
    pub fn dio() -> Self {
        return StandStats::new(200,10,15,85,40)
    }
    pub fn jotaro() -> Self {
        return StandStats::new(200,8,13,90,10)
    }
}

impl Default for StandStats {
    fn default() -> Self {
        return StandStats {
            hp: 100,
            speed: 5,
            attack: 10,
            accuracy: 90,
            dodge: 10,
            status: vec![],
        };
    }
}
