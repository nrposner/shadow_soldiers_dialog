use rand::Rng;
use std::collections::{HashMap, HashSet};

use shadow_soldiers_dialog::*;
mod isometric;
use isometric::IsometricSpace;

#[allow(unused_imports)]
use eframe::{egui, Frame};
#[allow(unused_imports)]
use egui::Id;

#[allow(dead_code)]
enum GameState {
    CharacterCreation,
    InGame,
    InventoryView,
    SkillManagement,
}

#[allow(dead_code)]
struct DialogueApp {
    current_text: String,
    player: Player,
    locations: HashMap<String, Location>, // All locations in the game
    current_location_id: String,          // Current location ID
    current_dialogue_id: Option<String>,  // Current dialogue ID, or None if not in a dialogue
    state: GameState,
    previous_dialogue_id: Option<String>,
    current_time: Time,
    isometric_space: IsometricSpace,
}

impl Default for DialogueApp {
    fn default() -> Self {
        Self {
            current_text: "Welcome!".to_string(),
            player: Player {
                tech: 3,
                arts: 3,
                bur: 3, //short for bureaucracy
                und: 3, //short for underworld
                checkmate_mod: 0,
                rocketry_mod: 0,
                pathology_mod: 0,
                civic_engineering_mod: 0,
                apparatchik_mod: 0,
                quota_mod: 0,
                robot_mod: 0,
                dossier_mod: 0,
                delusion_mod: 0,
                lens_mod: 0,
                opera_mod: 0,
                transcendence_mod: 0,
                gunsmoke_mod: 0,
                prohibition_mod: 0,
                gizmo_mod: 0,
                oldtime_religion_mod: 0,
                items: vec![],
                xp: 0,
                skill_points: 0,
                dialogues_entered: HashSet::new(),
                flags: HashSet::new(),
            },
            locations: create_locations(),
            current_location_id: "Vestibule".to_string(), // Start in the Vestibule
            current_dialogue_id: Some("Start".to_string()), // Start with the "Start" dialogue
            state: GameState::CharacterCreation,
            previous_dialogue_id: None,
            current_time: Time {
                day: 1,
                hour: 3,
                minute: 30,
            },
            isometric_space: IsometricSpace {
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct Time {
    day: i32,
    hour: i32,
    minute: i32,
}

impl Time {
    fn _increase(&mut self, added_minutes: i32) {
        let days_increased: i32 = added_minutes / 1440;

        let hours_increased: i32 = (added_minutes - 1440 * days_increased) / 60;

        let minutes_increased: i32 =
            (added_minutes - 1440 * days_increased) - (60 * hours_increased);

        self.day += days_increased;
        self.hour += hours_increased;
        self.minute += minutes_increased;

        if self.minute > 59 {
            self.minute = self.minute % 60;
            self.hour += 1;
        }

        if self.hour > 23 {
            self.hour = 0;
            self.day += 1;
        };
    }
}

fn _roll_dice() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=6), rng.gen_range(1..=6))
}

// Challenge logic
fn _handle_challenge(player: &Player, option: &DialogueOption) -> bool {
    if let Some(challenge_attribute) = &option.challenge_attribute {
        if let Some(challenge_number) = option.challenge_number {
            let attribute_value = match challenge_attribute.as_str() {
                "checkmate" => player.checkmate(),
                "rocketry" => player.rocketry(),
                "pathology" => player.pathology(),
                "civic engineering" => player.civic_engineering(),
                "apparatchik" => player.apparatchik(),
                "quota" => player.quota(),
                "robot" => player.robot(),
                "dossier" => player.dossier(),
                "delusion" => player.delusion(),
                "lens" => player.lens(),
                "opera" => player.opera(),
                "transcendence" => player.transcendence(),
                "gunsmoke" => player.gunsmoke(),
                "prohibition" => player.prohibition(),
                "gizmo" => player.gizmo(),
                "oldtime religion" => player.oldtime_religion(),
                _ => 0,
            };

            let (die1, die2) = _roll_dice();
            let roll_sum = die1 + die2;

            println!("You rolled: {} + {} = {}", die1, die2, roll_sum);

            if die1 == 6 && die2 == 6 {
                println!("Double sixes! Automatic success.");
                return true;
            } else if die1 == 1 && die2 == 1 {
                println!("Double ones! Automatic failure.");
                return false;
            }

            let total = roll_sum + attribute_value;
            if total >= challenge_number {
                println!(
                    "Success! You needed {}, and you got {}.",
                    challenge_number, total
                );
                return true;
            } else {
                println!(
                    "Failure. You needed {}, but you got {}.",
                    challenge_number, total
                );
                return false;
            }
        }
    }
    false
}

#[allow(dead_code)]
struct Player {
    tech: i32,
    arts: i32,
    bur: i32, //short for bureaucracy
    und: i32, //short for underworld
    checkmate_mod: i32,
    rocketry_mod: i32,
    pathology_mod: i32,
    civic_engineering_mod: i32,
    apparatchik_mod: i32,
    quota_mod: i32,
    robot_mod: i32,
    dossier_mod: i32,
    delusion_mod: i32,
    lens_mod: i32,
    opera_mod: i32,
    transcendence_mod: i32,
    gunsmoke_mod: i32,
    prohibition_mod: i32,
    gizmo_mod: i32,
    oldtime_religion_mod: i32,
    items: Vec<String>,
    xp: i32,
    skill_points: i32,
    dialogues_entered: HashSet<String>,
    flags: HashSet<String>,
}

#[allow(dead_code)]
impl Player {
    fn checkmate(&self) -> i32 {
        self.tech + self.checkmate_mod
    }

    fn rocketry(&self) -> i32 {
        self.tech + self.rocketry_mod
    }

    fn pathology(&self) -> i32 {
        self.tech + self.pathology_mod
    }

    fn civic_engineering(&self) -> i32 {
        self.tech + self.civic_engineering_mod
    }

    fn apparatchik(&self) -> i32 {
        self.bur + self.apparatchik_mod
    }

    fn quota(&self) -> i32 {
        self.bur + self.quota_mod
    }

    fn robot(&self) -> i32 {
        self.bur + self.robot_mod
    }

    fn dossier(&self) -> i32 {
        self.bur + self.dossier_mod
    }

    fn delusion(&self) -> i32 {
        self.arts + self.delusion_mod
    }

    fn lens(&self) -> i32 {
        self.arts + self.lens_mod
    }

    fn opera(&self) -> i32 {
        self.arts + self.opera_mod
    }

    fn transcendence(&self) -> i32 {
        self.arts + self.transcendence_mod
    }

    fn gunsmoke(&self) -> i32 {
        self.und + self.gunsmoke_mod
    }

    fn prohibition(&self) -> i32 {
        self.und + self.prohibition_mod
    }

    fn gizmo(&self) -> i32 {
        self.und + self.gizmo_mod
    }

    fn oldtime_religion(&self) -> i32 {
        self.und + self.oldtime_religion_mod
    }

    fn total_points(&self) -> i32 {
        self.tech + self.arts + self.bur + self.und
    }

    fn remaining_points(&self) -> i32 {
        12 - self.total_points()
    }

    fn is_valid(&self) -> bool {
        self.tech >= 1
            && self.arts >= 1
            && self.bur >= 1
            && self.und >= 1
            && self.tech <= 6
            && self.arts <= 6
            && self.bur <= 6
            && self.und <= 6
            && self.total_points() == 12
    }

    fn add_xp(&mut self, amount: i32) {
        self.xp += amount;

        // Handle leveling up
        while self.xp >= 100 {
            self.xp -= 100; // Reset XP and preserve the overflow
            self.skill_points += 1; // Award skill points
            println!(
                "You gained a skill point! You now have {} skill points.",
                self.skill_points
            );
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            tech: 1,
            arts: 1,
            bur: 1, //short for bureaucracy
            und: 1, //short for underworld
            checkmate_mod: 0,
            rocketry_mod: 0,
            pathology_mod: 0,
            civic_engineering_mod: 0,
            apparatchik_mod: 0,
            quota_mod: 0,
            robot_mod: 0,
            dossier_mod: 0,
            delusion_mod: 0,
            lens_mod: 0,
            opera_mod: 0,
            transcendence_mod: 0,
            gunsmoke_mod: 0,
            prohibition_mod: 0,
            gizmo_mod: 0,
            oldtime_religion_mod: 0,
            items: vec![],
            xp: 0,
            skill_points: 0,
            dialogues_entered: HashSet::new(),
            flags: HashSet::new(),
        }
    }
}

struct DialogueEditorApp {
    dialogues: HashMap<String, Dialogue>, // Dialogues being edited
    selected_dialogue: Option<String>,    // Currently selected dialogue ID
    temp_id: String,                      // Temporary field for editing dialogue ID
}

impl Default for DialogueEditorApp {
    fn default() -> Self {
        //Load in dialogues from the dedicated file
        //In the future, plan to transform this to load dialogues from room-specific files

        let (dialogues, temp_id) = initialize_dialogues("src/dialogues/clock.json").unwrap();

        Self {
            dialogues,
            selected_dialogue: None,
            temp_id,
        }
    }
}

impl eframe::App for DialogueEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dialogue Editor");

            // Add buttons for creating and saving dialogues
            ui.horizontal(|ui| {
                if ui.button("New Dialogue").clicked() {
                    self.create_dialogue();
                }
                if ui.button("Save").clicked() {
                    save_to_file(&self.dialogues, "src/dialogues/clock.json".to_string());
                }
            });

            // Use ScrollArea to wrap the rest of the content
            egui::ScrollArea::vertical()
                .id_salt("dialogue_scroll_area") // Provide a unique identifier for the scroll area
                .show(ui, |ui| {
                    // Display list of dialogues
                    self.display_dialogue_list(ui);

                    // Edit the selected dialogue
                    if let Some(selected_id) = &self.selected_dialogue {
                        if let Some(dialogue) = self.dialogues.get_mut(selected_id) {
                            if self.temp_id.is_empty() {
                                self.temp_id = selected_id.clone();
                            }
                            edit_dialogue(ui, selected_id, dialogue, &mut self.temp_id);
                        }
                    }
                });
        });
    }
}

impl DialogueEditorApp {
    fn create_dialogue(&mut self) {
        let id = create_dialogue(&mut self.dialogues); // Use the function from lib.rs
        self.temp_id = id.clone();
        self.selected_dialogue = Some(id);
    }

    fn _update_dialogues(&mut self, ui: &mut egui::Ui) {
        if let Some(selected_id) = &self.selected_dialogue {
            if let Some(dialogue) = self.dialogues.remove(selected_id) {
                let mut dialogue = dialogue;
                if self.temp_id.is_empty() {
                    self.temp_id = selected_id.clone(); // Initialize temp_id
                }

                if let Some(new_id) =
                    edit_dialogue(ui, selected_id, &mut dialogue, &mut self.temp_id)
                {
                    self.dialogues.insert(new_id.clone(), dialogue);
                    self.selected_dialogue = Some(new_id);
                    self.temp_id.clear();
                } else {
                    self.dialogues.insert(selected_id.clone(), dialogue);
                }
            }
        }
    }

    fn display_dialogue_list(&mut self, ui: &mut egui::Ui) {
        ui.label("Available Dialogues:");
        let mut to_delete = None;

        // Iterate through all dialogues
        for (id, _dialogue) in &self.dialogues {
            ui.horizontal(|ui| {
                // Button to edit a dialogue
                if ui.button(format!("Edit: {}", id)).clicked() {
                    self.selected_dialogue = Some(id.clone());
                }

                // Button to delete a dialogue
                if ui.button("Delete").clicked() {
                    to_delete = Some(id.clone());
                }
            });
        }

        // Delete the selected dialogue
        if let Some(id) = to_delete {
            self.dialogues.remove(&id);
        }
    }
}

// adding clap functionality so I can specify which dialogue file to access from main

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Dialogue Editor",
        options,
        Box::new(|_cc| Ok(Box::new(DialogueEditorApp::default()))),
    )
}

// if I wanted to be able to specify the dialogue file to load and modify, it would make the most sense to add a -- command after cargo run in which I could specify the name, likely not the full filepath. Then I'd have to preserve that and pass it into the update method somehow

// while I'm at it, ask ChatGPT what the hell that line above does, the |_cc|. Is that arbitrary? Is there a reason we call it that? Nothing is actually getting piped in, we're just calling the default. Is there perhaps a need for some filler in the closure?

// look more into the actual functioning and syntax of Box and eframe more generally, I'd like to be able to do that myself going forward

// going forward, I plan to put the isometric aspects on hold, instead focus on making this a functional text game, and then enabling images, more visual novel style
