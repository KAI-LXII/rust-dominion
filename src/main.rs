/*
SPUStudnet
12/15/2024
main.rs
Main module. Spawns the app, and handles most user keyboard input.
Keyboard input should really be delegated somewhere else, but I cannot make that big of a change right now.
*/

use std::{error::Error, io, io::stdout};
// Ratatui! an excellent user-interface library, used to display TUIs in terminals.
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
    restore,
};

// Import needed modules from both self and dominion library.
mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, WelcomeButton, GameSection},
    ui::ui,
};

use dominion_library::game::board::CardSet;
use dominion_library::player::player::PlayerUIInterface;

/**
 * Main method
 * Returns an error if the execution stopped with issues.
 */
fn main() -> Result<(), Box<dyn Error>> {
    // Install color_eyre.
    // Color_eyre is a color hook/ error handling object used by ratatui,
    // So if the program needs to panic, the terminal is still recovered and usable after the panic.
    color_eyre::install()?;

    // Enable raw mode so keyboard input can be handled
    enable_raw_mode()?;
    
    // Create an alternate screen.
    execute!(stdout(), EnterAlternateScreen)?;
    
    // Set the panic hook so terminal access can be restored in case of failure.
    set_panic_hook();

    // Create the backend using Crossterm (Fine-grained terminal control library)
    let backend = CrosstermBackend::new(stdout());

    // Create the terminal
    let mut terminal = Terminal::new(backend)?;


    // Start the application.
    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);


    // The app is done, disable and return terminal mode.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/**
 * In case of panic, boilerplate code for panic handling.
 */
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore(); // ignore any errors as we are already failing
        hook(panic_info);
    }));
}

/**
 * run_app
 * Runs the app given the app data structure and the terminal.
 * This main loop currently handles all user input.
 */
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {

    // Main app loop.
    loop {
        // Draw the UI for the terminal
        terminal.draw(|f| ui(f, app))?;

        // If we can read a key:
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not Press, as we don't care about them.
                continue;
            }
            match key.code {
                // On CTRL+C, exit.
                KeyCode::Char('c') => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        return Ok(true);
                    }
                },
                // If the player presses q, bring up an exit dialog.
                // This is a layered screen, so if q and an error are shown,
                // Error gets a lower priority than quit.
                KeyCode::Char('q') => {
                    app.prev_screen = Some(app.current_screen);
                    app.current_screen = CurrentScreen::Exiting
                    
                }
                _ => {}
            }
            // Match the screen we're in.
            match app.current_screen {
                // If we're in the welcome screen, use up, down to navigate, and enter to select.
                CurrentScreen::Welcome => match key.code {
                    KeyCode::Up => {
                        app.toggle_menu_button();
                    }
                    KeyCode::Down => {
                        app.toggle_menu_button();
                    }
                    KeyCode::Enter => match app.welcome_data.selected_button {
                        // Press enter, play the game, or run the exit dialog.
                        WelcomeButton::Play => {
                            app.current_screen = CurrentScreen::Playing
                        }
                        WelcomeButton::Exit => {
                            app.prev_screen = Some(CurrentScreen::Welcome);
                            app.current_screen = CurrentScreen::Exiting
                        }
                    }
                    // Note: Throughout the rest of this code, you'll see this symbol.
                    // It basically means "If there's no match, do nothing".
                    // It's all over the place because there's a lot of keycodes, and typing them out would be a pain.
                    _ => {}
                }
                CurrentScreen::Results => match key.code {
                    // If we're on the results page, any key closes the game.
                    _ => {
                        return Ok(true);
                    }
                }
                CurrentScreen::Exiting => match key.code {
                    // If we're on the exit dialog,
                    // Press y or n to exit, or restore the screen.
                    KeyCode::Char('y') =>{
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        match app.prev_screen {
                            // If there's a prev screen, go to it.
                            Some(x) => {
                                app.current_screen = x;
                                app.prev_screen = None;
                            }
                            None => {
                                // If there's no prev screen, go to the welcome screen.
                                app.current_screen = CurrentScreen::Welcome;
                            }
                        }
                    }
                    _ => {}
                }
                // Playing area: Actually playing the game.
                CurrentScreen::Playing => match app.game_nav_data.current_game_section {
                    // Depending on the player area, change the movement style.
                    GameSection::PlayerCards => handle_playercards_nav(app, key.code),
                    GameSection::Kingdom => handle_kingdom_nav(app, key.code),
                    GameSection::PlayerButtons => handle_player_buttons(app, key.code),
                    GameSection::ErrorPopup => match key.code {
                        // In case of error, restore previous screen once the player presses enter.
                        KeyCode::Enter => {
                            app.game_nav_data.restore_before_error();
                            app.update_items.update_player_stats;
                        }
                        _ => {}
                    },
                    GameSection::DescriptionPopup => match key.code {
                        _ => {
                            // If we're opening a card description, once a keycode is pressed, go back to the game.
                            app.game_nav_data.current_game_section = app.game_nav_data.prev_game_section.clone().unwrap();
                        }
                    }
                    _ => {}

                }
                _ => {}
            }
        }
    }
}


/**
 * PlayerCards nav.
 * This navigates the player's hand.
 */
fn handle_playercards_nav(app: &mut App, code:KeyCode) {
    match code {
        KeyCode::Up => {
            // When pressing up, go to the kingdom section
            // Go to zero, zero as we're not sure which cards exist or don't.
            app.game_nav_data.current_game_section = GameSection::Kingdom;
            app.game_nav_data.kingdom_column = 0;
            app.game_nav_data.kingdom_row = 0;
        }
        KeyCode::Right => {
            // If pressing right, if we're going to run off the end, go into the playerbutton interface.
            if app.game.get_player_character().get_hand().len() < (app.game_nav_data.card_selected + 2).into() {
                app.game_nav_data.current_game_section = GameSection::PlayerButtons;
            }
            else {
                // If we're not running off the end, just move right.
                app.game_nav_data.card_selected += 1;
            }
        }
        KeyCode::Left => {
            // If we're about to run off the rails to the left, go to the player buttons section.
            if app.game_nav_data.card_selected == 0 {
                app.game_nav_data.current_game_section = GameSection::PlayerButtons;
                app.game_nav_data.card_selected = app.game.get_player_character().get_hand().len() as u8 - 1;
            }
            else {
                // Otherwise, just move left.
                app.game_nav_data.card_selected -= 1;
            }
        }
        KeyCode::Enter => {

            // Upon pressing enter, play the card in hand.
            app.play_as_player(app.game_nav_data.card_selected.into());
            if app.game.get_player_character().get_hand().len() == 0 {
                // If there's no more cards, move to the playerButtons menu.
                app.game_nav_data.current_game_section = GameSection::PlayerButtons;
            } else {
                // If there's other cards, move to neighbors.
                if app.game_nav_data.card_selected == 0 {
                    app.game_nav_data.card_selected = app.game_nav_data.card_selected + 1;
                }
                else {
                    app.game_nav_data.card_selected = app.game_nav_data.card_selected - 1;
                }
                
            }
        },
        KeyCode::Char('?') => {
            // If we press ? in the hand, show a card description.
            app.game_nav_data.prev_game_section = Some(app.game_nav_data.current_game_section.clone());
            app.game_nav_data.current_game_section = GameSection::DescriptionPopup;
            let transition = app.game.get_player_character().get_hand().get(app.game_nav_data.card_selected as usize).unwrap();
            app.game_nav_data.card_describe = transition.get_description().clone();

        },
        _ => {}
    }
}

/**
 * handle_kingdom_nav
 * Handle navigation of the kingdom section
 * Currently does not let you select areas of the kingdom, as action cards don't exist.
 */
fn handle_kingdom_nav(app: &mut App, code:KeyCode) {
    match code {
        // When we move down,
        KeyCode::Down => {
            if app.game_nav_data.kingdom_row == 3 {
                // If we're at the edge of the kingdom rows, move into the player buttons (As we don't know if cards exist)
                app.game_nav_data.current_game_section = GameSection::PlayerButtons;
            }
            else if app.game_nav_data.kingdom_row == 2 {
                if app.game_nav_data.kingdom_column != 0 {
                    app.game_nav_data.kingdom_column = 0
                }
                app.game_nav_data.kingdom_row += 1;
            } else {
                // Otherwise, just move down.
                app.game_nav_data.kingdom_row += 1;

            }
            
        },
        KeyCode::Right => {
            // Loop back if we move all the way to the right.
            if app.game_nav_data.kingdom_row == 0 || app.game_nav_data.kingdom_row == 1 {
                if app.game_nav_data.kingdom_column == 2 {
                    app.game_nav_data.kingdom_column = 0;
                }
                else {
                    app.game_nav_data.kingdom_column += 1;
                }
            } else if  app.game_nav_data.kingdom_row == 2 {
                if app.game_nav_data.kingdom_column == 4 {
                    app.game_nav_data.kingdom_column = 0;
                }
                else {
                    app.game_nav_data.kingdom_column += 1;
                }
            }
            else {
                if app.game_nav_data.kingdom_column == 4 {
                    app.game_nav_data.kingdom_column = 0;
                }
                else {
                    app.game_nav_data.kingdom_column += 1;
                }
            }
            
        },
        KeyCode::Left => {
            // Loop back if we move all the way to the left.
            if app.game_nav_data.kingdom_row == 0 || app.game_nav_data.kingdom_row == 1 {
                if app.game_nav_data.kingdom_column == 0 {
                    app.game_nav_data.kingdom_column = 2;
                }
                else {
                    app.game_nav_data.kingdom_column -= 1;
                }
            }
            else if  app.game_nav_data.kingdom_row == 2 {
                if app.game_nav_data.kingdom_column == 0 {
                    app.game_nav_data.kingdom_column = 4;
                }
                else {
                    app.game_nav_data.kingdom_column -= 1;
                }
            } 
            else {
                if app.game_nav_data.kingdom_column == 0 {
                    app.game_nav_data.kingdom_column = 4;
                }
                else {
                    app.game_nav_data.kingdom_column -= 1;
                }
            }
            
        },
        KeyCode::Up => {
            // Move to player buttons if we go up, off the edge.
            if app.game_nav_data.kingdom_row == 0 {
                app.game_nav_data.current_game_section = GameSection::PlayerButtons;
            } else if app.game_nav_data.kingdom_row == 2 {
                app.game_nav_data.kingdom_row -= 1;
                if app.game_nav_data.kingdom_column > 2 {
                    app.game_nav_data.kingdom_column = 2;
                }
            } 
            else {
                app.game_nav_data.kingdom_row -= 1;
            }
            
        },
        KeyCode::Enter =>  {
            // On enter, buy a card, and show the error if there is one.
            let error;
            match app.game_nav_data.kingdom_row {
                
                0 => {
                    error = app.game.get_player_mut_character().buy_ui_card(app.game_nav_data.kingdom_column.into(), CardSet::Treasures);
                }
                1 => {
                    error = app.game.get_player_mut_character().buy_ui_card(app.game_nav_data.kingdom_column.into(), CardSet::Victories);
                }
                _ => {
                    if app.game_nav_data.kingdom_column > 5 {
                        error = app.game.get_player_mut_character().buy_ui_card((app.game_nav_data.kingdom_column - 5).into(), CardSet::Kingdoms);
                    } else {
                        error = app.game.get_player_mut_character().buy_ui_card((app.game_nav_data.kingdom_column).into(), CardSet::Kingdoms);
                    }
                }
            }

            match error {
                Some(e) => {
                    app.handle_error(e);                                    
                }
                None => {}
            }

            
                
            
        },
        KeyCode::Char('?') => {
            // If ? is pressed, show a popup of the card's description.
            app.game_nav_data.prev_game_section = Some(app.game_nav_data.current_game_section.clone());
            app.game_nav_data.current_game_section = GameSection::DescriptionPopup;

            let error;
            // Match the column go get the description.
            match app.game_nav_data.kingdom_row {
                
                0 => {
                    error = app.game.get_pile_desc(app.game_nav_data.kingdom_column.into(), CardSet::Treasures);
                }
                1 => {
                    error = app.game.get_pile_desc(app.game_nav_data.kingdom_column.into(), CardSet::Victories);
                }
                _ => {
                    if app.game_nav_data.kingdom_column > 5 {
                        error = app.game.get_pile_desc((app.game_nav_data.kingdom_column - 5).into(), CardSet::Kingdoms);
                    } else {
                        error = app.game.get_pile_desc((app.game_nav_data.kingdom_column).into(), CardSet::Kingdoms);
                    }
                }
            }

            // If there's an error, handle it.
            match (error) {
                Ok(desc) => {
                    app.game_nav_data.card_describe = String::from(desc);
                }
                Err(x) => {
                    app.handle_error(x);
                }
            }
            

        },
        _ => {}
    }
}


/**
 * Handle the input for the player buttons on the right-hand side.
 */
fn handle_player_buttons(app: &mut App, code: KeyCode) {
    match code {
        // If moving right, go the farthest-left carrd on the selection area. (loop around)
        KeyCode::Right => {
            if app.game.get_player_character().get_hand().len() > 0 {
                app.game_nav_data.current_game_section = GameSection::PlayerCards;
                app.game_nav_data.card_selected = 0;
            }
        },
        // If moving left, go to the card nearest the buttons.
        KeyCode::Left => {
            if app.game.get_player_character().get_hand().len() > 0 {
                app.game_nav_data.current_game_section = GameSection::PlayerCards;
                app.game_nav_data.card_selected = app.game.get_player_character().get_hand().len() as u8 - 1;
            }
            
        },
        // If moving down, toggle.
        KeyCode::Down => {
            app.game_nav_data.toggle_player_buttons();
        },
        // If moving up, if we're at the top, go to the kingdom cards.
        // Otherwise, just go up one.
        KeyCode::Up => {
            let m = app.game_nav_data.at_or_go_top();
            if m {
                app.game_nav_data.current_game_section = GameSection::Kingdom;
                app.game_nav_data.kingdom_column = 0;
                app.game_nav_data.kingdom_row = 0;
            }
        }
        KeyCode::Enter => {
            // Good example of helper methods that should be thoughout this section but aren't.
            app.handle_player_button_press();
            app.update_items.update_player_stats;
        }
        _ => {}
    }
}