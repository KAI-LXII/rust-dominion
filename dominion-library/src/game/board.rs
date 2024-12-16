/*
SPUStudnet
12/15/2024
board.rs
Defines the main game board with piles of kingdom cards, victory cards, treasure cards.
*/

use crate::card_manager::card::Card;
use crate::card_manager::card_structures::Pile;
use crate::game::game_errors::*;

/**
 * CardSet
 * The cards are divided into four rows:
 * The victories
 * Treasures
 * Kingdoms row 1
 * Kingdoms row 2
 * This is an enum to say which set a caller method is referring to, for clarity's sake.
 */
#[derive (Clone)]
pub enum CardSet {
    Victories,
    Treasures,
    Kingdoms
}

/**
 * Board
 * This is a board struct filled with vectors of piles.
 * It stores the kingdom set as one vector for simplicity,
 *  And has an (currently) unused trash pile.
 */
pub struct Board {
    pub(crate) victory_cards: Vec<Pile>,
    pub(crate) treasure_cards: Vec<Pile>,
    pub(crate) kingdom_set: Vec<Pile>,
    pub(crate) trash: Vec<Box<dyn Card>>
}


/**
 * Board implementation
 */
impl Board {

    /**
     * get_mut_pile
     * Get a mutable pile from the board.
     * Needed if you're trying to pop cards off of it.
     * This could probably be substituted with a pick_up_card method,
     * but this works O.k.
     */
    pub(self) fn get_mut_pile(&mut self, index: usize, c: CardSet) -> Option<&mut Pile> {
        // Match the card to its section.
        match c {
            CardSet::Kingdoms => {
                let foo = self.kingdom_set.get_mut(index);
                return foo;
            },
            CardSet::Treasures => {
                let foo = self.treasure_cards.get_mut(index);
                return foo;
            },
            CardSet::Victories => {
                let foo = self.victory_cards.get_mut(index);
                return foo;
            }
            // Get at the index, and return it.

        }
        
    }

    /**
     * get_pile
     * Immutable repeat of the above function
     * Useful if you're just trying to access information.
     * Probably can be substituted with a method returning a custom info struct, but this saves on that complexity.
     */
    pub(self) fn get_pile(& self, index: usize, c: CardSet) -> Option<&Pile> {
        // Match the card to its section.
        match c {
            CardSet::Kingdoms => {
                let foo = self.kingdom_set.get(index);
                return foo;
            },
            CardSet::Treasures => {
                let foo = self.treasure_cards.get(index);
                return foo;
            },
            CardSet::Victories => {
                let foo = self.victory_cards.get(index);
                return foo;
            }
            // Get at the index, and return it.

        }
        
    }

    /**
     * check_ending
     * This function is meant to be called at the end of every turn.
     * checks if the game has ended, and either the province pile is out, or two piles are empty.
     */
    pub(crate) fn check_ending(&self) -> bool {
        // Check the province pile.
        if self.victory_cards[2].is_empty() {
            return true;
        }

        // Initialize a counter accross ALL piles.
        let mut counter: u8 = 0;
        // Check the treasures.
        for pile in &self.treasure_cards {
            if pile.is_empty() {
                counter += 1;
            }
        }

        // Check the victories.
        for pile in &self.victory_cards {
            if pile.is_empty() {
                counter += 1;
            }
        }

        // Check the kingdoms.
        for pile in &self.kingdom_set {
            if pile.is_empty() {
                counter += 1;
            }


        }

        if counter >= 2 {
            return true;
        }

        return false;
    }
}

/**
 * PlayerInterface
 * This, in theory, is the only methods with which the player should interact with the board.
 * Realistically, the player needs a LOT more information, so the ui layer currently has ownership, and accesses properties directly.
 */
pub trait PlayerInterface {
    fn buy_card(self: &mut Self, index: usize, c: CardSet) -> Result<Box<dyn Card>, GameErrors>;
    fn get_card_price(&self, index: usize, c: CardSet) -> Result<i32, GameErrors>;
    fn get_pile_desc(&self, index: usize, c:CardSet) -> Result<String, GameErrors>;
}

/**
 * Implementation of PlayerInterface for Board.
 */
impl PlayerInterface for Board {
    /**
     * Buy card method.
     */
    fn buy_card(self: &mut Self, index: usize, c: CardSet) -> Result<Box<dyn Card>, GameErrors> {
        // Get the pile from the index and cardset.
        let pile_op = self.get_mut_pile(index, c);

        // If there's a card, return it.
        // Otherwise, show the error.
        match pile_op {
            Some(x) => {
                // The question mark is shorthand for "If there's an error, return it as the result"
                let card = x.pick_up_card()?;
                return Ok(card);
            }
            None => {
                return Err(GameErrors::CardNotFoundError(CardNotFoundError {index: index}))
            }
        }

    }

    /**
     * Get the price of a card.
     */
    fn get_card_price(&self, index: usize, c: CardSet) -> Result<i32, GameErrors> {
        // Get the pile
        let pile_op = self.get_pile(index, c);
        match pile_op {
            Some(x) => {
                // return it.
                return Ok(x.get_card_price());
            }
            None => {
                // Pile can't be found, return an error.
                return Err(GameErrors::CardNotFoundError(CardNotFoundError {index: index}))
            }
        }
    }

    /**
     * Get the description of a pile.
     */
    fn get_pile_desc(&self, index: usize, c:CardSet) -> Result<String, GameErrors> {
        let pile_op = self.get_pile(index, c);
        match pile_op {
            Some(x) => {
                // return it.
                return Ok(x.get_card_description());
            }
            None => {
                // Pile can't be found, return an error.
                return Err(GameErrors::CardNotFoundError(CardNotFoundError {index: index}))
            }
        }
    }
}

