// Let's create an interface to programatically make valid dialogues, and perhaps even visualize their flow in a conversation

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use dialoguer::{Input, Select};


pub fn load_dialogues(file_path: &str) -> HashMap<String, Dialogue> {
    // Read the JSON file into a string
    let file_content = fs::read_to_string(file_path)
        .expect("Failed to read the dialogue JSON file");
    
    // Parse the JSON string into a HashMap
    let dialogues_result: Result<HashMap<String, Dialogue>, _> = serde_json::from_str(&file_content);

    match dialogues_result {
        Ok(mut dialogues) => {
            // Validate and apply defaults to each Dialogue
            for (_, dialogue) in dialogues.iter_mut() {
                validate_and_fill_defaults(dialogue);
            }
            dialogues
        },
        Err(_) => {
            eprintln!("Error parsing dialogues. Using default dialogue.");
            let mut dialogues = HashMap::new();
            dialogues.insert("Default".to_string(), Dialogue::default());
            dialogues
        }
    }
}

fn validate_and_fill_defaults(dialogue: &mut Dialogue) {
    // Ensure `speaker` and `intro` are not empty
    if dialogue.speaker.is_empty() {
        dialogue.speaker = Dialogue::default().speaker;
    }
    if dialogue.intro.is_empty() {
        dialogue.intro = Dialogue::default().intro;
    }

    // Apply defaults for `options`
    if dialogue.options.is_empty() {
        dialogue.options = vec![DialogueOption::default()];
    } else {
        for option in dialogue.options.iter_mut() {
            if option.description.is_empty() {
                option.description = DialogueOption::default().description;
            }
            if option.success_dialogue.is_none() {
                option.success_dialogue = DialogueOption::default().success_dialogue;
            }
        }
    }

    // Validate `PassiveCheck` (all fields must be present)
    dialogue.passive_check.retain(|check| {
        if check.skill.is_empty() || check.target <= 0 || check.speaker.is_none() {
            eprintln!("Invalid PassiveCheck found and removed: {:?}", check);
            false // Remove invalid checks
        } else {
            true
        }
    });
}



pub fn main_menu(dialogues: &mut HashMap<String, Dialogue>) {
    loop {
        let save_path = "src/dialogues.json".to_string();
        println!("=== Dialogue Editor ===");
        let options = vec!["Create Dialogue", "Edit Dialogue", "View Dialogues", "Save to File", "Exit"];
        let selection = Select::new().items(&options).default(0).interact().unwrap();

        match selection {
            0 => create_dialogue(dialogues),
            1 => edit_dialogue(dialogues),
            2 => view_dialogues(dialogues),
            3 => save_to_file(dialogues, save_path),
            4 => break,
            _ => println!("Invalid selection"),
        }
    }
}

fn create_dialogue(dialogues: &mut HashMap<String, Dialogue>) {
    let id: String = Input::new().with_prompt("Enter Dialogue ID").interact().unwrap();
    let speaker: String = Input::new().with_prompt("Enter Speaker").interact().unwrap();
    let intro: String = Input::new().with_prompt("Enter Intro Text").interact().unwrap();

    let new_dialogue = Dialogue {
        speaker,
        intro,
        options: vec![],
        passive_check: vec![],
        xp_reward: None,
        is_hidden: false,
        time: None,
    };

    dialogues.insert(id, new_dialogue);
    println!("Dialogue created successfully.");
}

fn edit_dialogue(dialogues: &mut HashMap<String, Dialogue>) {
    let ids: Vec<String> = dialogues.keys().cloned().collect();
    if ids.is_empty() {
        println!("No dialogues available to edit.");
        return;
    }

    let selection = Select::new().items(&ids).default(0).interact().unwrap();
    let id = &ids[selection];
    let dialogue = dialogues.get_mut(id).unwrap();

    dialogue.speaker = Input::new()
        .with_prompt("Edit Speaker")
        .default(dialogue.speaker.clone())
        .interact()
        .unwrap();
    dialogue.intro = Input::new()
        .with_prompt("Edit Intro Text")
        .default(dialogue.intro.clone())
        .interact()
        .unwrap();

    println!("Dialogue updated successfully.");
}

fn view_dialogues(dialogues: &HashMap<String, Dialogue>) {
    for (id, dialogue) in dialogues {
        println!("ID: {}\nSpeaker: {}\nIntro: {}\n", id, dialogue.speaker, dialogue.intro);
    }
}

pub fn save_to_file(dialogues: &HashMap<String, Dialogue>, file_path: String) {
    // Load existing dialogues
    let mut existing_dialogues = if let Ok(content) = std::fs::read_to_string(&file_path) {
        serde_json::from_str::<HashMap<String, Dialogue>>(&content).unwrap_or_default()
    } else {
        HashMap::new() // Start with an empty HashMap if the file doesn't exist
    };

    // Merge the current dialogues into the existing ones
    existing_dialogues.extend(dialogues.clone());

    // Serialize the merged dialogues
    let json = serde_json::to_string_pretty(&existing_dialogues).expect("Failed to serialize dialogues");
    std::fs::write(&file_path, json).expect("Failed to write to file");

    println!("Dialogues saved successfully to {}", file_path);
}












#[derive(Clone)]
pub struct Location {
    pub name: String,
    pub dialogues: HashMap<String, Dialogue>,
    pub exits: Vec<String>, // Names of other locations you can move to
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DialogueOption {
    pub description: String,
    pub challenge_attribute: Option<String>,
    pub challenge_number: Option<i32>,
    pub success_dialogue: Option<String>,
    pub failure_dialogue: Option<String>,
    pub item_to_pickup: Option<String>,
    pub visible_when: Option<String>,
    pub flags: Option<Vec<String>>,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dialogue {
    pub speaker: String,
    pub intro: String,
    pub options: Vec<DialogueOption>,
    pub passive_check: Vec<PassiveCheck>, // New field for passive dialogue checks
    pub xp_reward: Option<i32>,
    pub is_hidden: bool,
    pub time: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PassiveCheck {
    pub skill: String,          // The player's skill to check
    pub target: i32,            // The number to check against
    pub success_text: Option<String>, // Text to display on success (Optional)
    pub failure_text: Option<String>, // Text to display on failure (Optional)
    pub speaker: Option<String>, // The speaker, who will be the same in both success and failure cases
}

impl Default for DialogueOption {
    fn default() -> Self {
        DialogueOption {
            description: "Continue".to_string(),
            challenge_attribute: None,
            challenge_number: None,
            success_dialogue: Some("Start".to_string()),
            failure_dialogue: None,
            item_to_pickup: None,
            visible_when: None,
            flags: None,
        }
    }
}

impl Default for Dialogue {
    fn default() -> Self {
        Dialogue {
            speaker: "Error".to_string(),
            intro: "No dialogue available.".to_string(),
            options: vec![
                DialogueOption::default(),
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
            time: Some(1),
        }
    }
}

impl Location {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dialogues: HashMap::new(),
            exits: vec![],
        }
    }

    pub fn add_dialogue(&mut self, id: String, dialogue: Dialogue) {
        self.dialogues.insert(id, dialogue);
    }

    pub fn add_exit(&mut self, exit: String) {
        self.exits.push(exit);
    }
}


//create defaults and use them, reduce space taken up
















pub fn create_locations() -> HashMap<String, Location> {
    let mut locations = HashMap::new();

    // Define sample dialogues for the Vestibule
    let mut vestibule_dialogues = HashMap::new();
    vestibule_dialogues.insert(
        "Start".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "The front door swings shut, cutting off the bitter wind like a scythe. You stand in the harsh light of a public apartment vestibule. A grid of mailboxes wait, closed, and a grandfather clock stands stout against the wall, like an elderly servant whose crooked back can't quite stand up to attention.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Inspect the grandfather clock.".to_string(),
                    success_dialogue: Some("InspectClock".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Look in the mailboxes.".to_string(),
                    success_dialogue: Some("VestibuleMailboxes".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Go to the first floor.".to_string(),
                    success_dialogue: Some("FirstFloor".to_string()),
                    ..Default::default()
                },
            ],
            is_hidden: false,
            ..Default::default()
        },
    );

    locations
}