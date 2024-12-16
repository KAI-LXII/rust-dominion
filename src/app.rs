/*
SPUStudnet
12/15/2024
card_manager.rs
Reference module, required by rust to reference modules in folder structures.
*/

use dominion_library::game::game_errors::GameErrors;
use dominion_library::player::player::PlayerUIInterface;
use dominion_library::game::game_manager::GameManager as gm;
use dominion_library::player::phases::PlayerPhases;


/**
 * CurrentScreen
 * Enum representing the current, overarching screen that is being played.
 */
#[derive(Clone, Copy)]
pub enum CurrentScreen{
    Welcome,
    Playing,
    Exiting,
    Results
}

/**
 * App struct
 * All data needed by the frontend to do its job.
 */
pub struct App {
    pub game_nav_data: GameNavData,
    pub current_screen: CurrentScreen,
    pub welcome_data: WelcomeScreen,
    pub prev_screen: Option<CurrentScreen>,
    pub update_items: UpdateItems,
    pub game: gm,
    pub end_data: Option<Vec<(String, i32)>>
}


/**
 * App implementation
 * Ideally this would be filled in with a LOT more helper methods and segments
 * for control.
 * However, main unfortunetly got most of that job ad-hoc.
 */
impl App {
    /**
     * New
     * Creates a new app with blank data on the welcome screen.
     */
    pub fn new() -> App {
        App {
            game_nav_data: GameNavData::new(),
            current_screen: CurrentScreen::Welcome,
            welcome_data: WelcomeScreen::new(),
            prev_screen: None,
            update_items: UpdateItems::new(),
            game: gm::new(),
            end_data: None
        }
        
    }

    /**
     * toggle_menu_button
     * Used to move between the two buttons on the main menu.
     * Independent of what you press, you're moving to the next one.
     */
    pub fn toggle_menu_button(&mut self) {
        self.welcome_data.toggle_selected_button();
    }

    /**
     * Play card as player
     * Accessor method for UI.
     * Takes in an index for the hand, and plays the given card.
     */
    pub fn play_as_player(&mut self, index: u8) {
        let error = self.game.get_player_mut_character().play_ui_card(index.into());

        match error {
            Some(x) => match x {
                GameErrors::InvalidActionError(err) => {
                    self.game_nav_data.error_message = Some(err.action_attempted);
                    self.game_nav_data.current_game_section
             = GameSection::ErrorPopup;
                }
                _ => {
                    self.game_nav_data.error_message = Some(String::from("An unknown error occured."));
                    self.game_nav_data.current_game_section
             = GameSection::ErrorPopup;
                }
            },
            _ => {}

        }
    }

    /**
     * handle_errror
     * Displays an error screen with the error message if needed.
     * Screen needs to be switched by UI right now, unfortunetly.
     */
    pub fn handle_error(&mut self, error: GameErrors) {
        match error {
            GameErrors::CardNotFoundError(err) => {
                self.game_nav_data.error_message = Some(String::from(format!("Cannot find card at index: {}", err.index)));
            }
            GameErrors::InvalidActionError(err) => {
                self.game_nav_data.error_message = Some(String::from(err.action_attempted));
            }
            _ => {
                self.game_nav_data.error_message = Some(String::from("A known error occured. But we can't tell you what it is yet. Sorry :("));
                self.game_nav_data.current_game_section = GameSection::ErrorPopup;
            }
        }
        
        
    }

    /**
     * Handles the player buttons press
     * A good example of a helper method I'd make more of if I had the time.
     */
    pub fn handle_player_button_press(&mut self) {
        // Which one is selected, the end actions/play treasures button, or the end turn button?
        match self.game_nav_data.button_selected {
            0 => {
                // actions/play treasures button
                match self.game.get_player_character().get_phase() {
                    PlayerPhases::Action => {
                        self.game.get_player_mut_character().advance_ui_phase();
                    },
                    PlayerPhases::Buy => {
                        self.game.get_player_mut_character().play_all_treasures_ui();
                    },
                    _ => {}
                }
            },
            1 => {
                // end turn button
                self.game.get_player_mut_character().end_turn();

                self.end_data = self.game.check_ending();

                // If the game has ended, stop playing immediately instead of giving the CPU another turn.
                if self.end_data.is_some() {
                    return
                }

                // Play as the cpu.
                self.game.player2_basic_ai();

                // Check for the ending of the game.
                self.end_data = self.game.check_ending();
            },
            _ => {}
        };
    }
}


/**
 * Enum representing welcome button.
 */
pub enum WelcomeButton {
    Play,
    Exit
}

/**
 * GameSection
 * Which section of the game is currently being manipulated.
 */
#[derive(PartialEq, Clone)]
pub enum GameSection {
    PlayerCards,
    PlayerButtons,
    Kingdom,
    SelectPopup,
    ErrorPopup,
    DescriptionPopup
}

/**
 * GameNavData
 * Indexes and pointers referring to which buttons are currently highlighted.
 */
pub struct GameNavData {
    // Which section of the game is being scrolled
    pub current_game_section: GameSection,
    // What's the previous section we were on?
    // (Used for popups and errors)
    pub prev_game_section: Option<GameSection>,

    // The card in hand that is being selected
    pub card_selected: u8,

    // The player button that is being selected.
    pub button_selected: u8,

    // The kingdom row that is being moved accross
    pub kingdom_row: u8,

    // The kingdom column that is being moved accross.
    pub kingdom_column: u8,

    // The description of a card, passed up for display purposes.
    pub card_describe: String,

    // The error messages, if there is one.
    pub error_message: Option<String>,

    // Unused. Used for selecting a card to gain/discard/etc.
    pub selection_message: Option<String>
}


/**
 * GameNavData
 * Struct implementation representing navigation data, where the player is scrolling.
 */
impl GameNavData {
    pub fn new() -> GameNavData {
        GameNavData {
            current_game_section: GameSection::PlayerCards,
            prev_game_section: None,
            card_selected: 0,
            button_selected: 0,
            kingdom_row: 0,
            kingdom_column: 0,
            error_message: None,
            selection_message: None,
            card_describe: String::from(""),
        }
        

    }

    // Restore the game to its previous state.
    pub fn restore_before_error(&mut self) {
        match &self.prev_game_section {
            Some(x) => {
                self.current_game_section = x.clone();
            }
            None => {
                self.current_game_section = GameSection::PlayerCards;
                self.card_selected = 0;

            }
        }
    }

    // Toggle the playerbutton that the player is selecting.
    pub fn toggle_player_buttons(&mut self) {
        match self.button_selected {
            0 => self.button_selected = 1,
            1 => self.button_selected = 0,
            _ => {}
        };
    }

    // Niche but nice helper method:
    // If the player's selection index is in the playerbuttons box,
    // and the player presses up, move to the top if it's on the bottom.
    // Otherwise, indicate that a different section can be moved to.
    pub fn at_or_go_top(&mut self) -> bool {
        match self.button_selected {
            0 => return true,
            1 => {self.button_selected = 0; return false},
            _ => {return false}
        };
    }
}

/**
 * UpdateItems
 * A struct representing which sections of the render should be re-rendered
 * This is done to prevent redundant rendering.
 * The effectiveness of this method is still in question, as it mostly went unused.
 */
pub struct UpdateItems {
    // Render the first time.
    pub first_render: bool,
    // Re-render the player stats page.
    pub update_player_stats: bool,
    // Re-render the player's card section
    pub update_player_cards: bool,
    // Re-render the update log.
    pub update_log: bool
}

impl UpdateItems {
    pub fn new() -> UpdateItems {
        UpdateItems {
            first_render: true,
            update_player_stats: false,
            update_player_cards: false,
            update_log: false,
        }
    }
}


/**
 * Welcome screen struct and implementation.
 */
pub struct WelcomeScreen {
    pub selected_button: WelcomeButton
}

impl WelcomeScreen {
    pub fn new() -> WelcomeScreen {
        WelcomeScreen {
            selected_button: WelcomeButton::Play
        }
    }

    pub fn toggle_selected_button(&mut self) {
        match &self.selected_button {
            WelcomeButton::Play => self.selected_button = WelcomeButton::Exit,
            WelcomeButton::Exit => self.selected_button = WelcomeButton::Play,
        };
    }
}