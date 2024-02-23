use super::items::armor::{Armor, ArmorType};
use super::items::hand_item::HandItem;
use super::items::hand_item::HandItemType;
use super::items::shield::Shield;
use super::items::weapon::Weapon;
use super::player::Player;
use super::store::Store;
use super::utils::{clear_screen, print_line, print_logo, slow_type};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::{io, process};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    player: Player,
    store: Store,
    state: GameState,
    current_week: u64,
    is_fight_week: bool,
}

#[derive(Serialize, Deserialize, Debug)]
enum GameState {
    MainMenu,
    Loading,
    InGame,
    GameOver,
    Inventory,
    Exit,
}

impl Game {
    pub fn new() -> Game {
        let mut player: Player = Player::new("Playername".to_string());
        let mut store: Store = Store::new();

        Game {
            player,
            store,
            state: GameState::MainMenu,
            current_week: 0,
            is_fight_week: false,
        }
    }

    fn advance_time(&mut self) {
        // Increment the week counter here
        self.current_week += 1;
        self.is_fight_week = self.current_week % 4 == 0;
    }

    pub fn main_loop(&mut self) {
        loop {
            match self.state {
                GameState::MainMenu => self.main_menu(),
                GameState::InGame => self.ludus_menu(),
                GameState::GameOver => self.end_game(),
                GameState::Loading => slow_type("Loading game"),
                GameState::Exit => self.exit(),
                GameState::Inventory => slow_type("Show inventory!"),
            }
        }
    }

    fn exit(&mut self) {
        // slow_type("Bye!");
        process::exit(0);
    }

    fn main_menu(&mut self) {
        print_line();
        print_logo();
        print_line();
        let main_menu_options = &["New Game", "Load Game", "Scores", "Exit"];

        let main_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Main Menu")
            .default(0)
            .items(&main_menu_options[..])
            .interact()
            .unwrap();
        match main_selection {
            0 => self.new_game(),
            1 => self.load_game_menu(), // Call the function to handle loading a game
            2 => slow_type("Showing high scores..."),
            3 => self.state = GameState::Exit,
            _ => unreachable!(),
        }
    }

    fn buy_freedom(&mut self) {
        slow_type("As you approach the owner of the ludus, your heart races with a mix of hope and anxiety. In your hand is a pouch of coins, meticulously saved over the years, representing your earnest attempt to buy your own freedom.");
        if self.player.money >= 1000 {
            slow_type("To your immense relief, the owner nods in acceptance, a greedy glint in his eye as he takes the hefty pouch of coins. Your eyes light up with disbelief and gratitude. With the shackles of servitude finally broken, you step out of the ludus, a free man, walking into a new life filled with endless possibilities...");
            self.state = GameState::GameOver;
        } else {
            slow_type("With a deep breath, you present your offer, only to see a frown crease the owner's face. He weighs the coins with a dismissive glance and declares it insufficient, his words crushing your hopes like fragile leaves underfoot. You return to the confines of the ludus, and your dream of freedom slipping away like sand through your fingers.");
            self.state = GameState::InGame;
        }
    }

    fn player_info(&self) {
        self.player.player_info();
        self.player.equipment.to_pretty_table();
        clear_screen();
    }

    fn player_inventory(&mut self) {
        // self.player.player_inventory();
        let inventory_options = &["Weapons", "Shields", "Helmets", "Back to Ludus"];
        let inventory_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Inventory")
            .default(0)
            .items(&inventory_options[..])
            .interact()
            .unwrap();

        match inventory_selection {
            0 => {
                slow_type("Weapons");
                self.player.player_inventory_weapon();
                self.player_inventory_equip_weapon();
            }
            1 => {
                slow_type("Shields");
                self.player.player_inventory_shields();
                self.player_inventory_equip_shield();
            }
            2 => {
                slow_type("Armor");
                self.player.player_inventory_armor();
                self.player_inventory_equip_armor();
            }
            3 => self.ludus_menu(),
            _ => unreachable!(),
        }

        // clear_screen();
    }

    fn store_menu(&mut self) {
        // self.player.player_inventory();
        let inventory_options = &["Weapons", "Shields", "Helmets", "Back to Ludus"];
        let inventory_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Store")
            .default(0)
            .items(&inventory_options[..])
            .interact()
            .unwrap();

        match inventory_selection {
            0 => {
                slow_type("Weapons");
                self.store.print_all_weapons();
                self.buy_weapon_menu();
            }
            1 => {
                slow_type("Shields");
            }
            2 => {
                slow_type("Armor");
            }
            3 => self.ludus_menu(),
            _ => unreachable!(),
        }

        // clear_screen();
    }

    fn buy_weapon_menu(&mut self) {
        let back_option = "Back to Store";

        let mut weapon_names: Vec<String> = self
            .store
            .weapons
            .iter()
            .map(|weapon| weapon.name.clone())
            .collect();

        weapon_names.push(back_option.to_string());

        let store_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Buy")
            .default(0)
            .items(&weapon_names)
            .interact()
            .unwrap();

        if store_selection == weapon_names.len() - 1 {
            // The player chose the "Back" option
            self.store_menu();
        } else {
            let selected_weapon = self.store.weapons[store_selection].clone();
            if self.player.money < selected_weapon.price {
                slow_type(format!("Can not buy: {}", selected_weapon.name).as_str());
                slow_type(
                    format!(
                        "Not enought money: Required {}  You have {}",
                        selected_weapon.price, self.player.money
                    )
                    .as_str(),
                );
            } else {
                let name = &selected_weapon.name.clone();
                self.player.money -= &selected_weapon.price;
                self.store.remove_weapon(store_selection);
                self.player
                    .inventory
                    .add_hand_item(HandItem::Weapon(selected_weapon));
                slow_type(format!("Purchased: {}", name).as_str());
            }
        }
        // clear_screen();
    }

    fn player_inventory_equip_weapon(&mut self) {
        let back_option = "Back to Inventory";

        let mut weapon_names: Vec<String> = self
            .player
            .inventory
            .hand_items
            .iter()
            .filter_map(|item| match item {
                HandItem::Weapon(weapon) => Some(weapon.name.clone()),
                _ => None, // Ignore items that are not weapons
            })
            .collect();

        weapon_names.push(back_option.to_string());

        let inventory_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Equip")
            .default(0)
            .items(&weapon_names)
            .interact()
            .unwrap();

        if inventory_selection == weapon_names.len() - 1 {
            // The player chose the "Back" option
            self.player_inventory();
        } else {
            println!("Equipping: {}", weapon_names[inventory_selection]);
            let weapons: Vec<&Weapon> = self
                .player
                .inventory
                .hand_items
                .iter()
                .filter_map(|item| match item {
                    HandItem::Weapon(weapon) => Some(weapon),
                    _ => None,
                })
                .collect();
            let selected_weapon = weapons[inventory_selection].clone();
            if self.player.strength < selected_weapon.req_strength {
                slow_type(format!("Can not equip: {:?}", selected_weapon).as_str());
                slow_type(
                    format!(
                        "Strength Required: {} Current Strength {}",
                        selected_weapon.req_strength, self.player.strength
                    )
                    .as_str(),
                );
            } else {
                slow_type(format!("Equipping: {:?}", selected_weapon).as_str());
                self.player.equipment.equip_weapon(selected_weapon);
            }
        }
        // clear_screen();
    }

    fn player_inventory_equip_shield(&mut self) {
        let back_option = "Back to Inventory";

        let mut shield_names: Vec<String> = self
            .player
            .inventory
            .hand_items
            .iter()
            .filter_map(|item| match item {
                HandItem::Shield(shield) => Some(shield.name.clone()),
                _ => None, // Ignore items that are not weapons
            })
            .collect();
        shield_names.push(back_option.to_string());

        let inventory_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Equip")
            .default(0)
            .items(&shield_names)
            .interact()
            .unwrap();

        if inventory_selection == shield_names.len() - 1 {
            // The player chose the "Back" option
            self.player_inventory();
        } else {
            let shields: Vec<&Shield> = self
                .player
                .inventory
                .hand_items
                .iter()
                .filter_map(|item| match item {
                    HandItem::Shield(shield) => Some(shield),
                    _ => None,
                })
                .collect();
            let selected_shield = shields[inventory_selection];
            if self.player.strength < selected_shield.req_strength {
                slow_type(format!("Can not equip: {:?}", selected_shield).as_str());
                slow_type(
                    format!(
                        "Strength Required: {} Current Strength {}",
                        selected_shield.req_strength, self.player.strength
                    )
                    .as_str(),
                );
            } else {
                slow_type(format!("Equipping: {:?}", selected_shield).as_str());
                self.player
                    .equipment
                    .equip_shield(selected_shield.to_owned());
            }
        }
        // clear_screen();
    }

    fn player_inventory_equip_armor(&mut self) {
        let back_option = "Back to Inventory";
        let mut armor_names: Vec<String> = self
            .player
            .inventory
            .armors
            .iter()
            .map(|armor| armor.name.clone())
            .collect();
        armor_names.push(back_option.to_string());

        let inventory_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Equip")
            .default(0)
            .items(&armor_names)
            .interact()
            .unwrap();

        if inventory_selection == armor_names.len() - 1 {
            // The player chose the "Back" option
            self.player_inventory();
        } else {
            let selected_armor = self.player.inventory.armors[inventory_selection].clone();
            if self.player.strength < selected_armor.req_strength {
                slow_type(format!("Can not equip: {}", armor_names[inventory_selection]).as_str());
                slow_type(
                    format!(
                        "Strength Required: {} Current Strength {}",
                        selected_armor.req_strength, self.player.strength
                    )
                    .as_str(),
                );
            } else {
                slow_type(format!("Equipping: {}", armor_names[inventory_selection]).as_str());
                self.player
                    .equipment
                    .equip_armor(self.player.inventory.armors[inventory_selection].clone());
            }
        }
        // clear_screen();
    }

    fn skip_fight(&mut self) {
        slow_type("You choose to skip this fight!");
        slow_type("Your lanista is not happy...");
        slow_type("Your are losing fame.");
        self.player.fame -= 10;
        //TODO logic for fame loss
        self.advance_time();
    }

    fn ludus_menu(&mut self) {
        print_line();
        println!("Week: {}", self.current_week);
        if self.is_fight_week {
            println!("This is a FIGHT week");
            let options = &[
                "Fight",
                "Skip fight",
                "Player Info",
                "Inventory",
                "Store",
                "Save Game",
                "To Main Menu",
            ];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Ludus")
                .default(0)
                .items(&options[..])
                .interact()
                .unwrap();
            match selection {
                0 => {
                    if self.player.injured {
                        slow_type("You are injured and can't fight this week.");
                        self.skip_fight();
                    }
                    let health_percentage =
                        self.player.health as f32 / self.player.max_health as f32;
                    if health_percentage > 0.5 {
                        self.fight();
                    } else {
                        slow_type("You are not ready to fight. Your health is to low.");
                        self.skip_fight();
                    }
                }
                1 => self.skip_fight(),
                2 => self.player_info(),
                3 => self.player_inventory(),
                4 => self.store_menu(),
                5 => {
                    self.save_game("save1.json").expect("Failed to save game."); // TODO: add different save files
                    slow_type("Game saved.");
                    self.ludus_menu();
                }
                6 => self.state = GameState::MainMenu,
                _ => unreachable!(),
            }
        } else {
            let options = &[
                "Player Info",
                "Train",
                "Rest",
                "Inventory",
                "Store",
                "Buy Freedom",
                "Save Game",
                "To Main Menu",
            ];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Ludus")
                .default(0)
                .items(&options[..])
                .interact()
                .unwrap();

            match selection {
                0 => self.player_info(),
                1 => self.train(),
                2 => self.rest(),
                3 => self.player_inventory(),
                4 => self.store_menu(),
                5 => self.buy_freedom(),
                6 => {
                    self.save_game("save1.json").expect("Failed to save game.");
                    slow_type("Game saved.");
                    self.ludus_menu();
                } // TODO: create save game feature
                7 => self.state = GameState::MainMenu,
                _ => unreachable!(),
            }
        }

        // clear_screen();
    }

    fn train(&mut self) {
        //TODO: add tiredness
        //TODO: add skill increase
        //TODO: add special moves
        if self.player.injured {
            slow_type("You can't train because of your injury... Try resting first.");
            self.state = GameState::InGame;
        } else {
            slow_type("You are training...");
            self.player.strength += 1;
            self.advance_time();
        }
    }

    fn rest(&mut self) {
        // TODO: figure out heal amout and injury heal process
        slow_type("You are resting. Restored 5 health");
        self.player.heal(5);
        println!("{}", self.player.health_bar());
        self.advance_time();
        self.state = GameState::InGame; //TODO: maybe add it to advance_time()
    }

    fn new_game(&mut self) {
        // TODO: add backstory of prisor of war
        // TODO: add skills setup during new game like
        clear_screen();
        slow_type("INTRODUCTION...");
        let text = "You found yourself in the arena...with a rusty sword in your hand and a terrifying enemy in front of you";
        slow_type(text);
        self.state = GameState::InGame;
        self.player.heal(self.player.max_health);
        self.fight();
    }

    fn end_game(&mut self) {
        slow_type(" ");
        slow_type("GAME OVER...");
        slow_type(" ");
        slow_type(" ");
        self.state = GameState::Exit;
    }

    fn beg_for_mercy(&mut self) {
        let mercy_option = &["Yes", "No"];

        let mercy_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Beg For Mercy?")
            .default(0)
            .items(&mercy_option[..])
            .interact()
            .unwrap();

        match mercy_selection {
            0 => self.show_mercy(),
            1 => {
                slow_type("Defeated yet defiant, you lie wounded in the Colosseum's arena, refusing to plead for mercy. Your pride remains unbroken, even in the face of imminent death.");
                slow_type("You feel the cold, sharp sting of your enemy's weapon and everything turns black...");
                self.state = GameState::GameOver;
            }
            _ => unreachable!(),
        }
    }

    fn show_mercy(&mut self) {
        // Embrace this new life with the strength of a warrior and the humility of one who has been given a second chance. Fight well, fight with honor, and let each victory bring you closer to the glory that now beckons.
        slow_type("You raising your trembling hands in a desperate plea for mercy. Your eyes, filled with a mix of fear and resignation, scan the sea of faces in the crowded stands, searching for a hint of compassion. ");
        let is_successfull = rand::thread_rng().gen_bool(0.5); //TODO: update chance based on popularity/fame
        if is_successfull {
            slow_type("To your surprise, amidst the tumult, a wave of compassion seems to sweep over the spectators, and they signal for mercy, sparing your life. With a heavy heart and a sense of shame, you slowly rise and exit the Colosseum, alive but forever marked by the day the crowd chose to let you live.");
            self.state = GameState::InGame;
        } else {
            slow_type("Your gaze, filled with fear and hope, meets a sea of unyielding faces, their chants for violence drowning out your plea.");
            slow_type("In that heart-wrenching moment, as the crowd's rejection seals your fate, you feel the cold, sharp sting of your enemy's weapon, ending your desperate fight for survival.");
            self.state = GameState::GameOver;
        }
    }

    fn fight(&mut self) {
        let mut enemy: Player = Player::new("Enemy1".to_string()); //TODO select enemy for each fight
        while self.player.health > 0 && enemy.health > 0 {
            clear_screen();
            println!("\t{} \t\t \t{}", self.player.name, enemy.name);
            println!("{}\t{}\n", self.player.health_bar(), enemy.health_bar());
            let action_options = &["Attack", "Block", "Wait"];
            let fight_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose your attack target:")
                .default(0)
                .items(&action_options[..])
                .interact()
                .unwrap();

            match fight_selection {
                0 => {
                    slow_type("Attacking...");
                    // Implement the logic for hitting the head
                    enemy.health -= 50;
                }
                1 => {
                    slow_type("Blocking...");
                    enemy.health -= 25;
                    // Implement logic for hitting the torso
                }
                2 => {
                    slow_type("Waiting...");
                    enemy.health -= 1;
                    // Implement logic for hitting the legs
                }
                _ => unreachable!(),
            }

            // Example enemy attack logic
            slow_type("Enemy attacks back!");
            self.player.take_damage(25); // Example damage from the enemy

            // Check if the player or enemy has been defeated
            if self.player.health <= 0 {
                self.beg_for_mercy();
                break;
            } else if enemy.health <= 0 {
                slow_type("Your decisive blow having vanquished your formidable enemy");
                slow_type("The crowd erupts in cheers, celebrating your triumph  as you emerge as the undisputed champion of the arena");

                self.player.money += 10; //TODO use enemy struct
                if self.player.victories == 0 {
                    // first victory
                    clear_screen();
                    slow_type("You are led out of the arena, not as a mere prisoner of war or a slave bound by chains, but as a warrior who has proven his mettle in the heat of combat.");
                    slow_type("The man in a silk cloth and two body guards approach you...");
                    slow_type("");
                    slow_type("[LANISTA] - You fought well today, beyond what was expected for a first fight.  Your victory is just the start.");
                    slow_type("He pauses, ensuring his words sink in before continuing.");
                    slow_type("[LANISTA] - Train hard, fight harder. Remember, you're here because I chose you—I see the gladiator in you. Your past is irrelevant; your future in the arena is what matters now.");
                    slow_type("The lanista's gaze hardens");
                    slow_type("[LANISTA] - I see in you a fighter worth the investment - prove me right, fight well and you will be rewarded.");
                    slow_type("Lanista leaves...Two of his bodyguards excort you to the Ludus.");
                }
                self.player.victories += 1;
                self.state = GameState::InGame;
                break;
            }
        }
        self.advance_time();
        clear_screen();
    }

    fn load_game_menu(&mut self) {
        let save_options = &["Save 1", "Save 2", "Save 3", "Back to Main Menu"];

        let save_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Load Game")
            .default(0)
            .items(&save_options[..])
            .interact()
            .unwrap();

        match save_selection {
            0 => {
                slow_type("Loading Save 1...");
                *self = Game::load_game("save1.json").unwrap_or_else(|_| {
                    slow_type("Failed to load game.");
                    Game::new()
                });
            }
            1 => slow_type("Loading Save 2..."),
            2 => slow_type("Loading Save 3..."),
            3 => self.main_menu(),
            _ => unreachable!(),
        }
    }

    fn save_game(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn load_game(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let game = serde_json::from_str(&contents)?;
        Ok(game)
    }
}
