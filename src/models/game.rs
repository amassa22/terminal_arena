use std::fs::{self, File};
use std::io::{Read, Write};
use std::time::Duration;
use std::{io, process, thread};

use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::items::armor::{Armor, ArmorType};
use super::items::hand_item::HandItem;
use super::items::item_type::ItemType;
use super::player::Player;
use super::items::shield::Shield;
use super::items::weapon::Weapon;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    player: Player,
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
        //todo remove after testing
        let shield = Shield::new("Basic Shield".to_string(), ItemType::SingleHand, 15, 10);
        let armor = Armor::new("Basic Helmet".to_string(), ArmorType::Helmet, 5, 10, 100, 5);
        let weapon2 = Weapon::new(
            "Advanced Rusty Sword".to_string(),
            ItemType::SingleHand,
            1,
            3,
            1,
        );
        player.inventory.add_armor(armor);
        player.inventory.add_hand_item(HandItem::Shield(shield));
        player.inventory.add_hand_item(HandItem::Weapon(weapon2));
        //

        Game {
            player,
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
        slow_type("Bye!");
        process::exit(0);
    }

    fn main_menu(&mut self) {
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
        clear_screen();
    }

    fn player_inventory(&self) {
        self.player.player_inventory();
        clear_screen();
    }

    fn skip_fight(&mut self) {
        slow_type("You choose to skip this fight!");
        slow_type("Your lanista is not happy...");
        slow_type("Your are losing fame.");
        self.player.fame -= 10;
        //todo logic for fame loss
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
                    let health_percentage =
                        self.player.health as f32 / self.player.max_health as f32;
                    if health_percentage > 0.05 {
                        self.fight();
                    } else {
                        slow_type("You are not ready to fight. Your health is to low.");
                        self.skip_fight();
                    }
                }
                1 => self.skip_fight(),
                2 => self.player_info(),
                3 => self.player_inventory(),
                4 => {
                    self.save_game("save1.json").expect("Failed to save game.");
                    slow_type("Game saved.");
                    self.ludus_menu();
                }
                5 => self.state = GameState::MainMenu,
                _ => unreachable!(),
            }
        } else {
            let options = &[
                "Player Info",
                "Train",
                "Rest",
                "Inventory",
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
                4 => self.buy_freedom(),
                5 => {
                    self.save_game("save1.json").expect("Failed to save game.");
                    slow_type("Game saved.");
                    self.ludus_menu();
                } // TODO: create save game feature
                6 => self.state = GameState::MainMenu,
                _ => unreachable!(),
            }
        }

        // clear_screen();
    }

    fn train(&mut self) {
        //todo: add tiredness
        //todo: add skill increase
        //todo: add special moves
        slow_type("You are training...");
        self.player.strength += 1;
        self.advance_time();
        self.state = GameState::InGame;
    }

    fn rest(&mut self) {
        //todo: add tiredness
        //todo: add skill increase
        //todo: add special moves
        slow_type("You are resting. Restored 5 health");
        self.player.heal(5);
        println!("{}", self.player.health_bar());
        self.advance_time();
        self.state = GameState::InGame;
    }

    fn new_game(&mut self) {
        // todo: add backstory of prisor of war
        // todo: add skills setup during new game like
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
        slow_type("You raising your trembling hands in a desperate plea for mercy. Your eyes, filled with a mix of fear and resignation, scan the sea of faces in the crowded stands, searching for a hint of compassion. ");
        let is_successfull = rand::thread_rng().gen_bool(0.5); //todo: update chance based on popularity/fame
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
        // let weapon = Weapon::new("Basic Rusty Sword".to_string(), ItemType::SingleHand, 1, 3, 1);
        let mut enemy: Player = Player::new("Enemy1".to_string()); //TODO select enemy for each fight
        while self.player.health > 0 && enemy.health > 0 {
            clear_screen();
            println!("\t{} \t\t \t{}", self.player.name, enemy.name);
            println!("{}\t{}\n", self.player.health_bar(), enemy.health_bar());
            let action_options = &["Head", "Torso", "Legs"];
            let fight_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose your attack target:")
                .default(0)
                .items(&action_options[..])
                .interact()
                .unwrap();

            match fight_selection {
                0 => {
                    slow_type("Hitting Head...");
                    // Implement the logic for hitting the head
                    enemy.health -= 50;
                }
                1 => {
                    slow_type("Hitting Torso...");
                    enemy.health -= 25;
                    // Implement logic for hitting the torso
                }
                2 => {
                    slow_type("Hitting Legs...");
                    enemy.health -= 1;
                    // Implement logic for hitting the legs
                }
                _ => unreachable!(),
            }

            // Example enemy attack logic
            // You can also implement a similar selection process for the enemy or randomize their actions
            slow_type("Enemy attacks back!");
            self.player.take_damage(25); // Example damage from the enemy

            // println!("Health: {}", style(self.player.health).red());
            // Check if the player or enemy has been defeated
            if self.player.health <= 0 {
                self.beg_for_mercy();
                break;
            } else if enemy.health <= 0 {
                slow_type("Your decisive blow having vanquished your formidable enemy");
                slow_type("The crowd erupts in cheers, celebrating your triumph  as you emerge as the undisputed champion of the arena");

                self.player.money += 10; //todo use enemy struct
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

fn slow_type(text: &str) {
    for c in text.chars() {
        print!("{}", c);
        std::io::stdout().flush().unwrap(); // Make sure character is printed immediately
        thread::sleep(Duration::from_millis(5));
    }
    println!();
}

fn print_line() {
    println!("{}", "═".repeat(30)); // Adjust the number of repetitions to fit your layout
}

fn clear_screen() {
    let term = Term::stdout();
    println!("Press any key to continue...");
    term.read_key().unwrap();
    term.clear_screen().unwrap();
}
