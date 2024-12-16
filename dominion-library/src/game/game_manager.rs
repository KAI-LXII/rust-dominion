/*
SPUStudnet
12/15/2024
game_manager.rs
Defines the game manager.
This is the only object passed to the UI layer, and it is enough to run the entire game, in theory.
Unfortunetly, additional logic currently requires pulling structs and objects from the rest of the game to call appropriate methods.
However, this provides a reasonable interface for playing a dominion game against a CPU.
*/


use std::{
    cell::RefCell, rc::Rc
};


use crate::game::board::PlayerInterface;
use crate::player::player::Player;
use crate::player::phases::PlayerPhases;
use crate::player::player::PlayerUIInterface;
use crate::game::board::Board;
use crate::game::pile_builder::*;
use crate::game::board::CardSet;
use crate::game::player_middleware::PlayerMiddleware;

use super::game_errors::GameErrors;

/**
 * GameManager struct
 * Contains all information about the game.
 * Holds control of the board, two players, and the middleware.
 */
pub struct GameManager {
    player1: Player,
    player2: Player,
    board: Rc<RefCell<Board>>,
    player_middleware: Rc<RefCell<PlayerMiddleware>>
}

impl GameManager {
    /**
     * New method.
     * Builds out all the information needed to create a brand-new game.
     */
    pub fn new() -> GameManager {

        // Create the board from the given piles, using helper functions.
        let mut board = Board {
            victory_cards: build_victory_piles(),
            treasure_cards: build_treasure_piles(),
            kingdom_set: Vec::new(),
            trash: Vec::new()
            
        };
        board.kingdom_set.push(build_moat_pile());
        board.kingdom_set.push(build_woodcutter_pile());
        board.kingdom_set.push(build_village_pile());
        board.kingdom_set.push(build_smithy_pile());
        board.kingdom_set.push(build_market_pile());

        // Create the middleware (Currently unused)
        let middle_cell = Rc::new(RefCell::new(PlayerMiddleware::new()));
        
        // Create the Boards refCell
        // Sidenote: This needs to be a refcell as it is shared between three objects, and each need immutable references, although never at the same time.
        let board_ref = Rc::new(RefCell::new(board));

        // Create player 1.
        let mut player1 = Player::new(build_default_player_deck(), board_ref.clone(), middle_cell.clone());
        player1.name = String::from("Player");

        // Create player 2.
        let mut player2 = Player::new(build_default_player_deck(), board_ref.clone(), middle_cell.clone());
        player2.name = String::from("CPU");

        // Make the GameManager we are going to return from all the above properties.
        let mut gm = GameManager {
            player1: player1,
            player2: player2,
            board: board_ref.clone(),
            player_middleware: middle_cell.clone()
        };

        // Shuffle the decks of both players.
        gm.player1.shuffle_deck();
        gm.player2.shuffle_deck();

        // Tell them to cleanup and draw their hand.
        gm.player1.cleanup_and_draw();
        gm.player2.cleanup_and_draw();

        // Start as player 1's turn.
        gm.player1.phase = PlayerPhases::Action;



        return gm;
    }

    /**
     * get_pile_data
     * Get data about all the piles
     * For use by the UI, as it needs to display this information.
     */ 
    pub fn get_pile_data(&self) -> PileData {
        let mut kingdom_data: Vec<(String, u8, i32)> = Vec::new();
        let mut victory_data: Vec<(String, u8, i32)> = Vec::new();
        let mut treasure_data: Vec<(String, u8, i32)> = Vec::new();


        // For each pile, get the cost, how many are left, and the name of the pile.
        for pile in 0..self.board.borrow().kingdom_set.len() {
            kingdom_data.push((self.board.borrow().kingdom_set.get(pile).unwrap().pile_name.clone(), 
            self.board.borrow().kingdom_set.get(pile).unwrap().cards_left, 
            self.board.borrow().get_card_price(pile, CardSet::Kingdoms).expect("Kingdom set asked for a card that didn't exist on creation.")));
        }

        for pile in 0..self.board.borrow().victory_cards.len() {
            victory_data.push((self.board.borrow().victory_cards.get(pile).unwrap().pile_name.clone(), 
            self.board.borrow().victory_cards.get(pile).unwrap().cards_left,
            self.board.borrow().get_card_price(pile, CardSet::Victories).expect("Victory set asked for a card that didn't exist on creation").clone()));
        }

        for pile in 0..self.board.borrow().treasure_cards.len() {
            treasure_data.push((self.board.borrow().treasure_cards.get(pile).unwrap().pile_name.clone(), 
            self.board.borrow().treasure_cards.get(pile).unwrap().cards_left,
            self.board.borrow().get_card_price(pile, CardSet::Treasures).expect("Treasure set asked for a card that didn't exist on creation").clone()));
        }

        // Return the newly created piledata struct.
        PileData {
            treasures: treasure_data,
            victories: victory_data,
            kingdom_set: kingdom_data
        }
    }

    // Get a mutable reference to player1 (Useful for UI)
    pub fn get_player_mut_character(&mut self) -> &mut impl PlayerUIInterface {
        return &mut self.player1;
    }

    // Get an immutable version to player1
    pub fn get_player_character(&self) -> &impl PlayerUIInterface {
        return &self.player1;
    }

    // Get the update log from the middleware.
    pub fn get_logs(&self) -> Vec<String> {
        self.player_middleware.borrow_mut().get_log().clone()
    }

    // Run the player2 ai.
    // This is effectively "Big money,"
    // The simplest dominion strategy that has a good chance of winning a game.
    pub fn player2_basic_ai(&mut self) {
        // Advance phase until we can play our treasures.
        while self.player2.phase != PlayerPhases::Buy {
            self.player2.advance_phase();
        }

        // Play all treasures
        let _ = self.player2.play_treasures();

        // If our buying power is greater than or equal to eight, buy a province.
        if self.player2.get_buying_power() >= 8 {
            self.player2.buy_card(2, CardSet::Victories);
        }
        else if self.player2.get_buying_power() >= 6 {
            // If the buying power is greater than or equal to six, and the victories are unaffordable, buy a gold.
            self.player2.buy_card(2, CardSet::Treasures);
        }
        else if self.player2.get_buying_power() >= 3 {
            // Otherwise, buy a sliver,
            self.player2.buy_card(1, CardSet::Treasures);
        }

        // Cleanup, and advance to idle.
        self.player2.cleanup_and_draw();

        self.player1.advance_phase();
    }

    // check_ending
    // At the end of every turn,this should be called.
    // If the game has ended, it returns a vector of scores with hardcoded names.
    // If the game hasn't ended, it returns nothing.
    pub fn check_ending(&mut self) -> Option<Vec<(String, i32)>> {
        if self.board.borrow().check_ending() == true {
            let p1_vp = self.player1.get_victory_points();
            let p2_vp = self.player2.get_victory_points();

            return Some(vec![(String::from("Player "), p1_vp), (String::from("CPU "), p2_vp)]);
        }
        else {
            None
        }
    }
    
    pub fn get_pile_desc(&mut self, index: usize, card_set: CardSet) -> Result<String, GameErrors> {
        return self.board.borrow().get_pile_desc(index, card_set)
    }
}


/**
 * PileData
 * Struct used to communicate to the UI the status of the piles.
 */
pub struct PileData {
    pub treasures: Vec<(String, u8, i32)>,
    pub victories: Vec<(String, u8, i32)>,
    pub kingdom_set: Vec<(String, u8, i32)>
}



