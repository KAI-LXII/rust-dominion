/*
SPUStudnet
12/15/2024
ui.rs
Main UI Rendering application and subfunctions.
Buckle up.
*/

// Import necessary libraries.
use dominion_library::{card_manager::card::Card, card_manager::card_properties::CardTypes, player::phases::PlayerPhases, player::player::PlayerUIInterface};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, 
    prelude::{Alignment, Stylize}, style::{Color, Modifier, Style}, 
    text::{Line, Span, Text}, 
    widgets::{Block, Borders, Clear, List, ListDirection, Paragraph, Padding, Wrap}, Frame
};

use crate::app::{App, CurrentScreen, WelcomeButton, GameSection};

// Base ui function:
// Render welcome, playing, exit, and end screen.
pub fn ui(frame: &mut Frame, app: &mut App) {
    welcome_screen(frame, app);
    playing_screen(frame, app);
    exit_screen(frame, app);
    end_screen(frame, app);
}

// end_screen
// Renders the end screen when there's endgame data.
fn end_screen(frame: &mut Frame, app: &mut App) {
    
    // Is there endgame data?
    match &app.end_data {
        Some(data) => {
            // Yes? Ok, clear the screen.
            frame.render_widget(Clear, centered_rect(100, 100, frame.area()));

            // Create a popup block to show the endgame dialog, with a border.
            let exit_popup_block = Block::default()
                .title("Endgame Dialog")
                .borders(Borders::ALL)
                .style(Style::default());

            
            // Create the exit game text.
            let exit_text = vec![
                Line::from("The game is over!").style(Style::default().fg(Color::Blue)).alignment(Alignment::Center),
                Line::from("Score: ").alignment(Alignment::Center),
                Line::from(format!("{}: VP: {}", data.get(0).unwrap().0, data.get(0).unwrap().1)).alignment(Alignment::Center),
                Line::from(format!("{}: VP: {}", data.get(1).unwrap().0, data.get(1).unwrap().1)).alignment(Alignment::Center),
                Line::from(format!("Thank you for playing!")).alignment(Alignment::Center)
            ];
            // Put it in a paragraph widget.
            let exit_paragraph = Paragraph::new(exit_text)
                .block(exit_popup_block)
                .wrap(Wrap { trim: false });

            // Put that widget in a centered rectangle.
            let area = centered_rect(60, 25, frame.area());

            // Render the final widget.
            frame.render_widget(exit_paragraph, area);

            // Change the current screen.
            app.current_screen = CurrentScreen::Results;
        }
        // No? do nothing.
        None => {}
    }
}


/**
 * playing_screen
 * Playing screen logic
 * handles rendering the entire playing screen.
 */
fn playing_screen(frame: &mut Frame, app: &mut App) {
    // If we're in the playing screen,
    if let CurrentScreen::Playing = app.current_screen {
        
        // Split the board into vertical areas, one for the player, and one for the board.
        let [board_area, player_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(20)
        ]).areas(frame.area());


        // Split the board area into two sections: One for the board, one for the log.
        let [log_area, board_area] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70)
        ]).areas(board_area);

        // Render the buying area.
        render_buying_area(frame, board_area, app);

        // Render the logging area.
        if app.update_items.first_render | app.update_items.update_log {
            render_log(frame, log_area, app.game.get_logs());
        }

        // Layout the player area into subsections.
        let [player_stats_area, player_play_area, player_buttons_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(60), Constraint::Percentage(10)]).areas(player_area);

        // Render the player's buttons.
        render_player_buttons(frame, player_buttons_area, app);

        // Render the stats box that shows their buying power, points, and actions.
        if app.update_items.first_render | app.update_items.update_player_stats {
            render_stats_box(frame, player_stats_area, app.game.get_player_character(), String::from("Player Stats"));
        }

        // Render their list of cards.
        if app.update_items.first_render | app.update_items.update_player_cards {
            // let mut state = app.selected_card.borrow_mut();
            // let hand = CardContainer::get_hand_view(app.game.get_player_character().get_hand());
            // hand.render(player_play_area, frame.buffer_mut(), state);
            let card_count: usize = app.game.get_player_character().get_hand().len();
            let mut player_area_constraint_vec = vec![];

            for _ in 0..card_count {
                player_area_constraint_vec.push(Constraint::Ratio(1, card_count.try_into().unwrap()))
            }

            
            let card_layout= Layout::default().direction(Direction::Horizontal).constraints(player_area_constraint_vec).split(player_play_area);

            for i  in 0..card_count {
                render_card(frame, card_layout[i],  app.game.get_player_character().get_hand().get(i as usize).unwrap(), i == app.game_nav_data.card_selected.into() && app.game_nav_data.current_game_section == GameSection::PlayerCards)
            }
        }

        // If there's an error, render it (We render it last so it isn't overwritten by everything else.)
        if app.game_nav_data.current_game_section == GameSection::ErrorPopup {
            let game_error_popup_block = Block::default()
            .title("Error")
            .borders(Borders::ALL)
            .style(Style::default());

            let error_text = Text::styled(
                app.game_nav_data.error_message.as_ref().unwrap().clone() + " - Press enter to continue",
                Style::default().fg(Color::Red),
            );

            let error_paragraph = Paragraph::new(error_text)
            .block(game_error_popup_block)
            .wrap(Wrap { trim: false });


            let area = centered_rect(30, 25, frame.area());

            frame.render_widget(Clear, area);
            frame.render_widget(error_paragraph, area);
        }

        // If the player has requested a card description, render it.
        render_card_description(frame, app);


    }
}

/**
 * render_card_description.
 * If there's a description popup, we render it.
 */
fn render_card_description(frame: &mut Frame, app: &mut App) {
    if app.game_nav_data.current_game_section == GameSection::DescriptionPopup {
        // Clear the screen
        frame.render_widget(Clear, centered_rect(100, 100, frame.area()));

        // Create a description block
        let description_block = Block::default()
            .title("Card Description")
            .borders(Borders::ALL)
            .style(Style::default());

        
        // Put the description block into a paragraph widget
        let paragraph = Paragraph::new(Text::from(app.game_nav_data.card_describe.clone()))
            .block(description_block)
            .wrap(Wrap { trim: false });

        // Center than paragraph
        let area = centered_rect(60, 25, frame.area());

        // Render it.
        frame.render_widget(paragraph, area);
    }
}

/**
 * Render the buying area
 * Split the buying area into a 4x5 grid, rendering a paragraph in each one.
 * I would've done a card struct, but that's REALLY difficult.
 */
pub fn render_buying_area(frame: &mut Frame, area: Rect, app: &mut App) {
    // Split the layout vertically.
    let vert_layout = Layout::vertical([
        Constraint::Ratio(1, 4); 4
    ]).split(area).to_vec();

    // Split the layout horizontally.
    let mut horizontal_cards: Vec<Vec<Rect>> = vec![];
    for layout in vert_layout {
        horizontal_cards.push(Layout::horizontal([Constraint::Ratio(1, 5); 5]).split(layout).to_vec())
    }

    // Change the selected style to the lightblue one we've been using (Yes, there should be one, universal one but limits)
    let selected_style = Style::default().bg(Color::LightBlue);

    // Get data for the games piles.
    let data = app.game.get_pile_data();

    // For each item in the treasure section.
    for i in 0..data.treasures.len() {
        // Put the data in a paragraph, render it in the right box.
        // If the cursor is over that item, and this area is the active window, draw it as blue.
        let mut w = Paragraph::new(vec![Line::from(format!("{}", data.treasures[i].0)),
        Line::from(format!("Left in stock: {}", data.treasures[i].1)),
        Line::from(format!("Price: {}", data.treasures[i].2))]).style(Style::default().fg(Color::LightYellow));
        if app.game_nav_data.kingdom_row == 0 && app.game_nav_data.kingdom_column == i as u8 && app.game_nav_data.current_game_section == GameSection::Kingdom {
            w = w.style(selected_style);
        }
        frame.render_widget(w, horizontal_cards[0][i])
    }

    for i in 0..data.victories.len() {
        // Put the data in a paragraph, render it in the right box.
        // If the cursor is over that item, and this area is the active window, draw it as blue.
        let mut w = Paragraph::new(vec![Line::from(format!("{}", data.victories[i].0)),
        Line::from(format!("Left in stock: {}", data.victories[i].1)),
        Line::from(format!("Price: {}", data.victories[i].2))]).style(Style::default().fg(Color::LightGreen));
        if app.game_nav_data.kingdom_row == 1 && app.game_nav_data.kingdom_column == i as u8 && app.game_nav_data.current_game_section == GameSection::Kingdom {
            w = w.style(selected_style);
        }
        frame.render_widget(w, horizontal_cards[1][i])
    }

    for i in 0..data.kingdom_set.len() {
        // Put the data in a paragraph, render it in the right box.
        // If the cursor is over that item, and this area is the active window, draw it as blue.
        let mut w = Paragraph::new(vec![Line::from(format!("{}", data.kingdom_set[i].0)),
        Line::from(format!("Left in stock: {}", data.kingdom_set[i].1)),
        Line::from(format!("Price: {}", data.kingdom_set[i].2))]).style(Style::default().fg(Color::Gray));

        if i < 5 {
            if app.game_nav_data.kingdom_row == 2 && app.game_nav_data.kingdom_column == i as u8 && app.game_nav_data.current_game_section == GameSection::Kingdom {
                w = w.style(selected_style);
            }

            frame.render_widget(w, horizontal_cards[2][i])
        }
        else {
            if app.game_nav_data.kingdom_row == 3 && app.game_nav_data.kingdom_column == (i - 5) as u8 && app.game_nav_data.current_game_section == GameSection::Kingdom {
                w = w.style(selected_style);
            }
            frame.render_widget(w, horizontal_cards[3][i - 5])
        }
        
    }


}

/**
 * render_log
 * Render the game's overall log to the screen.
 */
fn render_log(frame: &mut Frame, area: Rect, logs: Vec<String>) {

    // Render all game log items as a list going top to bottom
    let list = List::new(logs)
    .block(Block::bordered().title("Log").title_alignment(Alignment::Left))
    .style(Style::new().white())
    .highlight_style(Style::new().italic())
    .direction(ListDirection::TopToBottom);
    frame.render_widget(list, area);
}

/**
 * render_player_buttons
 * Render the buttons that a player has. (End actions, end turn, play treasures)
 */
fn render_player_buttons(frame: &mut Frame, area: Rect, app: &App) {
    // Set up a layout for the buttons, along with some bocks and styles.
    let button_layout = Layout::vertical([Constraint::Ratio(1, 2); 2]).split(area);
    let button_block = Block::new().borders(Borders::ALL);
    let selected_style = Style::default().bg(Color::LightBlue);

    // Create the three paragraphs that will act as "Buttons"
    let mut actions_paragraph = Paragraph::new("End Actions").block(button_block.clone()).centered();
    let mut treasures_paragraph = Paragraph::new("Play all treasures").block(button_block.clone()).centered();
    let mut end_turn_paragraph = Paragraph::new("End Turn").block(button_block.clone()).centered();

    // If the window is being used, and the button is selected, change the style accordingly
    if app.game_nav_data.button_selected == 0 && app.game_nav_data.current_game_section == GameSection::PlayerButtons {
        actions_paragraph = actions_paragraph.style(selected_style);
        treasures_paragraph = treasures_paragraph.style(selected_style);
    }
    else if app.game_nav_data.current_game_section == GameSection::PlayerButtons {
        end_turn_paragraph = end_turn_paragraph.style(selected_style);
    }

    // Depending on the phase, show the "End Actions" and "Play treasures" buttons.
    if app.game.get_player_character().get_phase() == PlayerPhases::Action {
        
        frame.render_widget(actions_paragraph, button_layout[0]);
    }
    else if app.game.get_player_character().get_phase() == PlayerPhases::Buy {
        
        frame.render_widget(treasures_paragraph, button_layout[0])
    }

    frame.render_widget((end_turn_paragraph).centered(), button_layout[1])
    
}

/**
 * render_stats_box
 * Renders a box of stats that shows information about the user's game.
 */
fn render_stats_box(frame: &mut Frame, stat_area: Rect, player: &dyn PlayerUIInterface, title: String) {
    // Create all the strings from player get calls.
    let ac_string = format!("Actions: {}\n", player.get_actions());
    let vp_string = format!("Victory points: {}\n", player.get_victory_points());
    let bp_string = format!("Buying power: {}\n", player.get_buying_power());
    let phase_string = format!("Phase: {}\n", player.get_phase().to_string());
    let buys_string = format!("Buys: {}\n", player.get_buys().to_string());

    // Create the lines.
    let lines = Text::from(vec![
            Line::styled(phase_string, Style::default().fg(Color::LightBlue)),
            Line::styled(ac_string, Style::default().fg(Color::Gray)),
            Line::styled(vp_string, Style::default().fg(Color::LightGreen)),
            Line::styled(bp_string, Style::default().fg(Color::LightYellow)),
            Line::styled(buys_string, Style::default().fg(Color::Magenta))
    ]);

    // Create the lines as a paragraph.
    let widget = Paragraph::new(lines)
    .left_aligned();

    // Render the lines.
    render_borders(&widget, Borders::ALL, frame, stat_area, Some(title));

}

/**
 * Render a box with borders.
 * A nice little helper method that renders a paragraph with extra borders
 * Yes, I would've made more of these if I had planned/had time.
 */
fn render_borders(paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect, title: Option<String>) {
    let mut block = Block::new()
        .borders(border);

    match title {
        Some(t) => {
            block = block.title(t).title_alignment(Alignment::Left);
        }
        _ => {}
    }
    frame.render_widget(paragraph.clone().block(block), area);
}

/**
 * Render_card
 * Renders a card in the player's hand.
 */
fn render_card(frame: &mut Frame, card_area: Rect, card: &Box<dyn Card>, selected: bool) {
    // Split the given space into a space of exactly 20 in length.
    let card_layout = Layout::horizontal([
        Constraint::Length(20)
    ]).split(card_area);
    
    // Get the card's new rectangle it's going to be rendered in.
    let layout_item = card_layout.get(0);

    // Add borders. (This doesn't work, for some reason.)
    let mut outside_block = Block::new().borders(Borders::ALL);

    if selected {
        outside_block = outside_block.style(Style::default().bg(Color::LightBlue))
    }

    
    
    let horizontal_chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Ratio(1, 2),
        Constraint::Ratio(1, 2),
        Constraint::Length(3),
    ]);

    let [card_title, card_image, card_description, card_types] = horizontal_chunks.areas(*layout_item.unwrap());


    // Declare and render card title
    let card_title_block = Block::new().borders(Borders::all()).padding(Padding::top(0));
    let text = Line::from(Span::styled(card.get_name(), Style::default())).alignment(Alignment::Center);
    let lp = Paragraph::new(text).block(card_title_block);
    frame.render_widget(lp, card_title);


    // set the card art's background colors depending on the type of card.
    let mut bg_color = Style::default();
    if card.get_card_types().contains(&CardTypes::Victory) {
        bg_color = bg_color.fg(Color::LightGreen);
    }
    else if card.get_card_types().contains(&CardTypes::Action) {
        bg_color = bg_color.fg(Color::Gray);
    }
    else if card.get_card_types().contains(&CardTypes::Treasure) {
        bg_color = bg_color.fg(Color::LightYellow);
    }


    // Render Card art (There isn't any at this point.)
    let card_image_block = Block::new().borders(Borders::all()).style(bg_color);
    let asciiartparagraph = Paragraph::new(Line::from("Card art goes here.")).centered();
    frame.render_widget(asciiartparagraph.clone().block(card_image_block), card_image);

    //Render card description
    let card_description_block = Block::new().borders(Borders::all());
    let description = Paragraph::new(Line::from(card.get_description().clone())).wrap(Wrap {trim: true}).centered();
    frame.render_widget(description.clone().block(card_description_block), card_description);


    // Dynamically create the card type string: e.g Action - Reaction.
    let card_type_block = Block::new().borders(Borders::all());
    let mut card_type_string: String = String::default();
    let mut iter_first = true;
    for i in card.get_card_types() {
        if !iter_first {
            card_type_string += " - "
        }
        match i {
            CardTypes::Action => {
                card_type_string += "Action"
            }
            CardTypes::Victory => {
                card_type_string += "Victory"
            }
            CardTypes::Treasure => {
                card_type_string += "Treasure"
            }
            CardTypes::Reaction => {
                card_type_string += "Reaction"
            }
            CardTypes::Attack => {
                card_type_string += "Attack"
            }
        }
        iter_first = false;
    }

    // Center the paragraph text of type, and render it.
    let type_text = Paragraph::new(Line::from(card_type_string)).centered();
    frame.render_widget(type_text.clone().block(card_type_block), card_types);

    // Render the outside block we declared before. It must be rendered last to overwrite all other items, instead of the other way around.
    frame.render_widget(outside_block, *layout_item.unwrap());


}

/**
 * welcome_screen
 * Renders the welcome screen for the player.
 */
fn welcome_screen(frame: &mut Frame, app: &App) {
    if let CurrentScreen::Welcome = app.current_screen {

        // Divide the screen into three segments: the title, and the two buttons
        let _ = frame.render_widget(Clear, frame.area());
        let title_screen_blocks = Layout::default()
        .margin(3)
        .constraints([
            Constraint::Min(5),
            Constraint::Min(3),
            Constraint::Min(3),
        ])
        .split(frame.area());


        // Create the block that the title paragraph will be rendered inside.
        let popup_block = Block::default()
        .style(Style::default())
        .borders(Borders::ALL);


        // Create the Title and disclaimer text.
        let mut lines = vec![];
        lines.push(Line::from(
            Span::styled("Dominion - Console Version", Style::default().fg(Color::Blue))
        ));
        lines.push(Line::from(
            Span::styled("Original game by copyright holders. This version makes no claim of ownership, and its use is restricted for educational purposes.",
             Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        ));

        // Get the title text,
        let title_text = Text::from(lines);

        // Create the paragraph, bound to the inner block for borders.
        let title_paragraph = Paragraph::new(title_text)
            .block(popup_block)
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center);
        // Render inside the layout.
        frame.render_widget(title_paragraph, title_screen_blocks[0]);

        // Create the two buttons.
        let mut play_button = Block::default().borders(Borders::ALL);
        let mut exit_button = Block::default().borders(Borders::ALL);

        let selected_style = Style::default().bg(Color::LightBlue).fg(Color::Black);

        // Add style to them if they are selected.
        match app.welcome_data.selected_button {
            WelcomeButton::Play => play_button = play_button.style(selected_style),
            WelcomeButton::Exit => exit_button = exit_button.style(selected_style),
        }

        // Render the two buttons with text.
        let play_text = Paragraph::new("Play").block(play_button);
        frame.render_widget(play_text, title_screen_blocks[1]);

        let exit_test = Paragraph::new("Exit").block(exit_button);
        frame.render_widget(exit_test, title_screen_blocks[2]);
    }
}

/**
 * exit_screen
 * Render the exit request screen.
 */
fn exit_screen(frame: &mut Frame, app: &App) {
    if let CurrentScreen::Exiting = app.current_screen {

        // Create a new widget for the exit dialog.
        frame.render_widget(Clear, frame.area());
        let exit_popup_block = Block::default()
            .title("Exit dialog")
            .borders(Borders::ALL)
            .style(Style::default());

        
        // Create the exit text.
        let exit_text = Text::styled(
            "Are you sure that you want to exit? If a game is being played, it will not be saved! (y/n)",
            Style::default().fg(Color::Red),
        );

        // Add it to a paragrpah.
        let exit_paragraph = Paragraph::new(exit_text)
            .block(exit_popup_block)
            .wrap(Wrap { trim: false });

        // Center the paragraph using an area.
        let area = centered_rect(60, 25, frame.area());

        // Render the paragraph in that area.
        frame.render_widget(exit_paragraph, area);
    }
}

/**
 * Boilerplate method for rendering an image in the center by cutting the rendering area into chunks.
 */
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}