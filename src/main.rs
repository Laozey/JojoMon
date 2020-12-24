mod game_processing;
mod stand_data;

use std::collections::HashMap;
use std::{path::PathBuf, vec};

use game_processing::*;
use ggez::audio::*;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::run;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::*;
use ggez::mint::Vector2;
use ggez::nalgebra as na;
use ggez::timer::fps;
use ggez::{Context, ContextBuilder, GameResult};
use na::Point2;

use stand_data::*;


pub struct MyGame {
    // Your state here...
    theme_sound: Source,
    scene: u8,
    turn: u32,
    j1_data: StandInfo,
    j1_attacks: Vec<Attacks>,

    j2_data: StandInfo,
    j2_attacks: Vec<Attacks>,

    j1_selected_attacks: Attacks,
    j2_selected_attacks: Attacks,
    sounds: HashMap<Attacks, Source>,
}

impl MyGame {
    //TODO Racourcir ce code
    fn process(&mut self) {
        self.theme_sound.set_volume(0.1);
        if faster_than(&self.j1_data, &self.j2_data) {
            process_turn(
                &mut self.j1_data,
                &mut self.j2_data,
                &mut self.j1_attacks,
                &mut self.sounds,
            );
            process_turn(
                &mut self.j2_data,
                &mut self.j1_data,
                &mut self.j2_attacks,
                &mut self.sounds,
            );
        } else {
            process_turn(
                &mut self.j2_data,
                &mut self.j1_data,
                &mut self.j2_attacks,
                &mut self.sounds,
            );
            process_turn(
                &mut self.j1_data,
                &mut self.j2_data,
                &mut self.j1_attacks,
                &mut self.sounds,
            );
        }
        self.theme_sound.set_volume(1.0);
    }

    fn display_level_text(&self, context: &mut Context) -> GameResult<()> {
        // basic font
        let hello_world_font = Font::default();

        // Display j1 selected attack
        let mut j1_attack;
        match self.j1_selected_attacks {
            Attacks::Zawarudo(_) => j1_attack = Text::new("J1 selected Zawarudo !"),
            Attacks::Muda => j1_attack = Text::new("J1 selected Muda !"),
            Attacks::RoadRoller => j1_attack = Text::new("J1 selected Road Roller !"),
            Attacks::Charisme => j1_attack = Text::new("J1 selected Charisme !"),
            Attacks::MotherSoul => j1_attack = Text::new("J1 selected Mother Soul !"),
            Attacks::Ora => j1_attack = Text::new("J1 selected Ora !"),
            Attacks::Facture => j1_attack = Text::new("J1 selected Facture !"),
            Attacks::None => j1_attack = Text::new("Choose an attack !"),
        }
        j1_attack.set_font(hello_world_font, Scale::uniform(15.0));

        draw(
            context,
            &j1_attack,
            DrawParam::default().dest(Point2::new(20.0, 360.0)),
        )?;

        // Display j2 selected attack
        let mut j2_attack;
        match self.j2_selected_attacks {
            Attacks::Zawarudo(_) => j2_attack = Text::new("J2 selected Zawarudo !"),
            Attacks::Muda => j2_attack = Text::new("J2 selected Muda  !"),
            Attacks::RoadRoller => j2_attack = Text::new("J2 selected Road Roller !"),
            Attacks::Charisme => j2_attack = Text::new("J2 selected Charisme !"),
            Attacks::MotherSoul => j2_attack = Text::new("J2 selected Mother Soul !"),
            Attacks::Ora => j2_attack = Text::new("J2 selected Ora !"),
            Attacks::Facture => j2_attack = Text::new("J2 selected Facture !"),
            Attacks::None => j2_attack = Text::new("Choose an attack !"),
        }
        j2_attack.set_font(hello_world_font, Scale::uniform(15.0));

        draw(
            context,
            &j2_attack,
            DrawParam::default().dest(Point2::new(550.0, 360.0)),
        )?;

        // Display j1 attacks
        let j1_attacks = vec![
            self.j1_data.attack1,
            self.j1_data.attack2,
            self.j1_data.attack3,
            self.j1_data.attack4,
        ];
        let j1_attacks_location = vec![
            Point2::new(30.0, 405.0),
            Point2::new(150.0, 405.0),
            Point2::new(30.0, 500.0),
            Point2::new(150.0, 500.0),
        ];
        let mut j1_attacks_text: Vec<Text> = Vec::new();

        for attack in j1_attacks {
            match attack {
                Attacks::Zawarudo(_) => j1_attacks_text.push(Text::new("(R) Zawarudo")),
                Attacks::Muda => j1_attacks_text.push(Text::new("(A) Muda")),
                Attacks::RoadRoller => j1_attacks_text.push(Text::new("(Z) Road Roller")),
                Attacks::Charisme => j1_attacks_text.push(Text::new("(E) Charisme")),
                Attacks::MotherSoul => j1_attacks_text.push(Text::new("(E) Mother Soul")),
                Attacks::Ora => j1_attacks_text.push(Text::new("(A) Ora")),
                Attacks::Facture => j1_attacks_text.push(Text::new("(Z) Facture")),
                Attacks::None => j1_attacks_text.push(Text::new("")),
            }
        }

        let mut i = 0;

        for mut text in j1_attacks_text {
            text.set_font(hello_world_font, Scale::uniform(12.0));
            draw(
                context,
                &text,
                DrawParam::default().dest(j1_attacks_location[i]),
            )?;
            i += 1;
        }

        // Display j1 attacks
        let j2_attacks = vec![
            self.j2_data.attack1,
            self.j2_data.attack2,
            self.j2_data.attack3,
            self.j2_data.attack4,
        ];
        let j2_attacks_location = vec![
            Point2::new(560.0, 405.0),
            Point2::new(680.0, 405.0),
            Point2::new(560.0, 500.0),
            Point2::new(680.0, 500.0),
        ];
        let mut j2_attacks_text: Vec<Text> = Vec::new();

        for attack in j2_attacks {
            match attack {
                Attacks::Zawarudo(_) => j2_attacks_text.push(Text::new("(R) Zawarudo")),
                Attacks::Muda => j2_attacks_text.push(Text::new("(A) Muda")),
                Attacks::RoadRoller => j2_attacks_text.push(Text::new("(Z) Road Roller")),
                Attacks::Charisme => j2_attacks_text.push(Text::new("(E) Charisme")),
                Attacks::MotherSoul => j2_attacks_text.push(Text::new("(E) Mother Soul")),
                Attacks::Ora => j2_attacks_text.push(Text::new("(A) Ora")),
                Attacks::Facture => j2_attacks_text.push(Text::new("(Z) Facture")),
                Attacks::None => j2_attacks_text.push(Text::new("")),
            }
        }
        let mut i = 0;
        for mut text in j2_attacks_text {
            text.set_font(hello_world_font, Scale::uniform(12.0));
            draw(
                context,
                &text,
                DrawParam::default().dest(j2_attacks_location[i]),
            )?;
            i += 1;
        }

        // Display doc
        let mut help =
            Text::new("Try press A, Z, E or R...\nPress 'ctrl + key' to display tooltip");
        help.set_font(hello_world_font, Scale::uniform(15.0));
        draw(
            context,
            &help,
            DrawParam::default().dest(Point2::new(0.0, 0.0)),
        )?;

        let mut fps_text;
        if fps(context) as u64 > 300 {
            fps_text = Text::new("Fps > 300");
        } else {
            fps_text = Text::new(format!("Fps : {}", fps(context).round()).to_string());
        }
        fps_text.set_font(hello_world_font, Scale::uniform(15.0));
        draw(
            context,
            &fps_text,
            DrawParam::default().dest(Point2::new(730.0, 0.0)),
        )?;

        // Display turn
        let mut turn = Text::new(String::from(format!(
            "turn {}",
            ((self.turn / 2) + 1).to_string()
        )));
        turn.set_font(hello_world_font, Scale::uniform(30.0));
        draw(
            context,
            &turn,
            DrawParam::default().dest(Point2::new(355.0, 470.0)),
        )?;

        let mut j1_hp = Text::new(String::from("J1 hp : "));
        let hp = TextFragment::new(self.j1_data.hp.to_string()).color(Color::from_rgb(255, 30, 30));
        j1_hp.add(hp);
        j1_hp.set_font(hello_world_font, Scale::uniform(20.0));
        draw(
            context,
            &j1_hp,
            DrawParam::default().dest(Point2::new(20.0, 315.0)),
        )?;

        let mut j2_hp = Text::new(String::from("J2 hp : "));
        let hp = TextFragment::new(self.j2_data.hp.to_string()).color(Color::from_rgb(255, 30, 30));
        j2_hp.add(hp);
        j2_hp.set_font(hello_world_font, Scale::uniform(20.0));
        draw(
            context,
            &j2_hp,
            DrawParam::default().dest(Point2::new(690.0, 315.0)),
        )?;

        Ok(())
    }

    fn display_menu(&self, context: &mut Context) -> GameResult {
        let mut title = Text::new("Jojo-Mon");
        title.set_font(Font::default(), Scale::uniform(40.0));
        queue_text(
            context,
            &title,
            Point2::new(325.0, 10.0),
            Some(Color::from_rgb(255, 255, 255)),
        );

        let image = Image::new(context, "/uxu4w1zqggn51.png")?;
        draw(
            context,
            &image,
            DrawParam::default()
                .dest(Point2::new(130.0, 100.0))
                .scale(na::Vector2::new(0.5, 0.5)),
        )?;

        //Press Return to play...
        let mut t1 = Text::new("Press");
        t1.set_font(Font::default(), Scale::uniform(20.0));
        queue_text(
            context,
            &t1,
            Point2::new(300.0, 550.0),
            Some(Color::from_rgb(255, 255, 255)),
        );

        let mut t2 = Text::new("Return");
        t2.set_font(Font::default(), Scale::uniform(20.0));
        queue_text(
            context,
            &t2,
            Point2::new(355.0, 550.0),
            Some(Color::from_rgb(255, 200, 90)),
        );

        let mut t3 = Text::new("to play...");
        t3.set_font(Font::default(), Scale::uniform(20.0));
        queue_text(
            context,
            &t3,
            Point2::new(420.0, 550.0),
            Some(Color::from_rgb(255, 255, 255)),
        );

        draw_queued_text(context, DrawParam::default(), None, FilterMode::Linear)?;

        Ok(())
    }

    fn match_attacks(&self, player_attack: &Attacks, process_display: &mut Text) {
        let mut t2= TextFragment::new("with ");
        let mut t3= TextFragment::new("");
        let mut t4= TextFragment::new("");
        let mut t5= TextFragment::new("");
        match player_attack {
            Attacks::Zawarudo(_) => {
                t3 = TextFragment::new("Zawarudo ").color(Color::from_rgb(255, 200, 90));
                t4 = TextFragment::new("dealing ");
                t5 = TextFragment::new("0").color(Color::from_rgb(30, 255, 80));
            }
            Attacks::Muda => {
                t3 = TextFragment::new("Muda ").color(Color::from_rgb(255, 200, 90));
                t4 = TextFragment::new("dealing ");
                t5 = TextFragment::new("a lot of damage").color(Color::from_rgb(30, 255, 80));
            }
            Attacks::RoadRoller => {
                t3 = TextFragment::new("Road Roller ").color(Color::from_rgb(255, 200, 90));
                t4 = TextFragment::new("dealing ");
                t5 = TextFragment::new(format!(
                    "{} damage",
                    (self.j1_data.strength * 4).to_string()
                ))
                .color(Color::from_rgb(30, 255, 80));
            }
            Attacks::Charisme => {
                t3 = TextFragment::new("Charisme ").color(Color::from_rgb(255, 200, 90));
                t4 = TextFragment::new("receiving ");
                t5 = TextFragment::new("30 hp").color(Color::from_rgb(30, 255, 80));
            }
            Attacks::MotherSoul => {
                t2 = TextFragment::new("");
                t3 = TextFragment::new("sworning on his ");
                t4 = TextFragment::new("Mother's soul").color(Color::from_rgb(255, 200, 90));
            }
            Attacks::Ora => {
                t3 = TextFragment::new("Ora ").color(Color::from_rgb(255, 200, 90));
                t4 = TextFragment::new("dealing ");
                t5 = TextFragment::new("a lot of damage").color(Color::from_rgb(30, 255, 80));
            }
            Attacks::Facture => {
                t3 = TextFragment::new("Facture ").color(Color::from_rgb(255, 200, 90));
                t4 = TextFragment::new("dealing ");
                t5 = TextFragment::new(format!(
                    "{} damage",
                    (self.j1_data.strength * 4).to_string()
                ))
                .color(Color::from_rgb(30, 255, 80));
            }
            Attacks::None => (),
        }
        process_display.add(t2);
        process_display.add(t3);
        process_display.add(t4);
        process_display.add(t5);
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
       if !self.theme_sound.playing(){
           self.theme_sound.play()?;
       }
        Ok(())
    }

    fn key_up_event(&mut self, _context: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        // Menu
        if self.scene == 0 {
            if keycode == KeyCode::Return {
                self.scene = 1
            }
        }
        // Combat scene
        if self.scene == 1 {
            if (self.turn % 2) == 0 {
                // Player 1 turn ...
                match keycode {
                    k @ KeyCode::A | k @ KeyCode::Z | k @ KeyCode::E | k @ KeyCode::R => {
                        if keymods.contains(KeyMods::CTRL) {
                            // Fonction tool tip
                        } else {
                            self.j1_attacks.push(select_attack(&self.j1_data, k));
                            self.j1_selected_attacks = self.j1_attacks[0];
                            self.turn += 1;
                        }
                    }
                    _ => (),
                }
            } else {
                // Player 2 turn ...
                match keycode {
                    k @ KeyCode::A | k @ KeyCode::Z | k @ KeyCode::E | k @ KeyCode::R => {
                        if keymods.contains(KeyMods::CTRL) {
                            // Fonction tool tip
                        } else {
                            self.j2_attacks.push(select_attack(&self.j2_data, k));
                            self.j2_selected_attacks = self.j2_attacks[0];
                            self.turn += 1;
                            self.process();
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    // TODO Ajouter le text des attaques plus l'input associé
    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        if self.scene == 0 {
            clear(context, Color::from_rgb(120, 120, 120));
            self.display_menu(context)?;
        } else if self.scene == 1 {
            clear(context, Color::from_rgb(1, 14, 20));
            // Draw code here...
            let background = Image::new(context, "/ac4a0ea3d1691ee48f7b7680e829dd5d.png")?;
            draw(context, &background, DrawParam::default())?;

            let mut attack_meshes: Vec<Mesh> = Vec::new();

            // Button rect, color and offsets
            let mut rect = Rect::new(25.0, 400.0, 100.0, 75.0);
            let mut offset = Vector2 { x: 0.0, y: 0.0 };
            let mut rcolor = Color::from_rgb(255, 240, 100);
            let mut rcolor_switch: (u8, u8, u8) = (255, 240, 100);

            // Create a mesh then move the offset (UI)
            for i in 0..=7 {
                match i {
                    0 | 2 => offset.x = 120.0,
                    1 => {
                        offset.x = 410.0;
                        rcolor_switch = (255, 100, 100);
                    }
                    3 => {
                        offset.x = 0.0;
                        offset.y = 95.0;
                    }
                    4 | 6 => {
                        offset.x = -120.0;
                        offset.y = 0.0;
                    }
                    5 => {
                        offset.x = -410.0;
                        rcolor_switch = (255, 240, 100);
                    }
                    _ => (),
                };
                attack_meshes.push(
                    Mesh::new_rectangle(context, DrawMode::stroke(5.0), rect, rcolor)
                        .expect("Couldn't inisialise Mesh"),
                );
                // Adjustements...
                rcolor = Color::from_rgb(rcolor_switch.0, rcolor_switch.1, rcolor_switch.2);
                rect.translate(offset)
            }
            // Draw meshes
            for meshe in attack_meshes {
                draw(context, &meshe, DrawParam::default())?;
            }

            let line_mesh = MeshBuilder::new()
                .line(
                    &[Point2::new(0.0, 305.0), Point2::new(800.0, 305.0)],
                    5.0,
                    WHITE,
                )?
                .build(context)?;
            draw(context, &line_mesh, DrawParam::default())?;

            if self.turn % 2 == 0 && self.turn > 0 {
                let mut process_display = Text::new("");
                let t1 = TextFragment::new(self.j1_data.name.to_string())
                    .color(Color::from_rgb(30, 70, 255));
                process_display.add(t1);
                let t2 = TextFragment::new(" attack ");
                process_display.add(t2);
                self.match_attacks(&self.j1_selected_attacks, &mut process_display);
                let t6 = TextFragment::new("... ");
                process_display.add(t6);
                let t7 = TextFragment::new(self.j2_data.name.to_string())
                    .color(Color::from_rgb(255, 30, 30));
                process_display.add(t7);
                let t8 = TextFragment::new(" retaliat ");
                process_display.add(t8);
                self.match_attacks(&self.j2_selected_attacks, &mut process_display);
                let t12 = TextFragment::new("!");
                process_display.add(t12);
                process_display.set_font(Font::default(), Scale::uniform(13.0));
                draw(
                    context,
                    &process_display,
                    DrawParam::default().dest(na::Point2::new(20.0, 280.0)),
                )?;
            }

            let line_mesh = MeshBuilder::new()
                .line(
                    &[Point2::new(0.0, 345.0), Point2::new(800.0, 345.0)],
                    5.0,
                    WHITE,
                )?
                .build(context)?;
            draw(context, &line_mesh, DrawParam::default())?;

            self.display_level_text(context)?;
        
            let mut imagej1 = Image::new(context, "/HEYimHeroic_3DS_FACE-024_Matt-Wii.png")?;
            match self.j1_data.name.as_str() {
                "Dio" => imagej1 = Image::new(context, "/Eoh_DIO.png")?,
                "Jotaro" => imagej1 = Image::new(context, "/Jotaro_SC_Infobox_Manga.png")?,
                _ => (),
            }
            draw(
                context,
                &imagej1,
                DrawParam::default()
                    .dest(na::Point2::new(40.0, 70.0))
                    .scale(na::Vector2::new(0.5, 0.5)),
            )?;

            let mut imagej2 = Image::new(context, "/HEYimHeroic_3DS_FACE-024_Matt-Wii.png")?;
            match self.j2_data.name.as_str() {
                "Dio" => imagej2 = Image::new(context, "/Eoh_DIO.png")?,
                "Jotaro" => imagej2 = Image::new(context, "/Jotaro_SC_Infobox_Manga.png")?,
                _ => (),
            }
            draw(
                context,
                &imagej2,
                DrawParam::default()
                    .dest(na::Point2::new(600.0, 70.0))
                    .scale(na::Vector2::new(0.5, 0.5)),
            )?;
        }

        present(context)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // Make a Context
    let window_setup = WindowSetup::default()
        .title("Jojomon, Gotta Ora Ora Ora'em all!")
        .vsync(false);

    let (mut context, mut event_loop) = match ContextBuilder::new("Jojomon", "Quentin Epron")
        .window_mode(WindowMode::default())
        .window_setup(window_setup)
        .add_resource_path(PathBuf::from("./resources"))
        .build()
    {
        Ok(ctxbuilder) => ctxbuilder,
        Err(error) => panic!("Couldn't create ggez context : {}", error),
    };
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame {
        theme_sound:Source::new(&mut context,"/sound/GameTheme_sound.mp3").unwrap(),
        scene: 0,
        turn: 0,
        j1_data: StandInfo::dio(),
        j2_data: StandInfo::jotaro(),
        j1_attacks: Vec::new(),
        j2_attacks: Vec::new(),
        j1_selected_attacks: Attacks::None,
        j2_selected_attacks: Attacks::None,
        sounds: HashMap::new(),
    };
    attack_sound_load(&mut my_game.sounds, &mut context);
    Ok(match run(&mut context, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(error) => println!("Error occured: {}", error),
    })
}
