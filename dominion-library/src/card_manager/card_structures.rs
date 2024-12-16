/*
SPUStudnet
12/15/2024
card_structures.rs
Implementation of the complex "Pile" struct, representing a homogenous pile of cards.
*/

use crate::game::game_errors::OutOfCardsError;
use crate::card_manager::card::Card;

/**
 * Pile struct
 * A pile represents a stack of one card.
 * Instead of creating the cards at runtime and hogging memory resources, the pile contains a builder method, and returns one card at a time.
 */
pub struct Pile {
    pub pile_name: String,
    pub cards_left: u8,
    pub card_creator: fn() -> Box<dyn Card>
}

/**
 * Pile implementation
 */
impl Pile {
    /**
     * is_empty
     * Boolean accessor returning if the pile is empty or not.
     */
    pub fn is_empty(&self) -> bool {
        return self.cards_left == 0;
    }

    /**
     * pick_up_card
     * Pick up a card from the pile.
     * If there are no more cards, return an OutOfCards error.
     */
    pub fn pick_up_card(&mut self) -> Result<Box<dyn Card>, OutOfCardsError> {
        if self.cards_left > 0 {
            self.cards_left -= 1;
            let a: Box<dyn Card> = (self.card_creator)();
            Ok(a)
        } 
        else {
            Err(OutOfCardsError)
        }
    }

    /**
     * get_cards_left
     * Getter method for the number of cards left.
     */
    pub fn get_cards_left(&self) -> u8 {
        return self.cards_left;
    }

    /**
     * get_pile_name
     * Getter method for the name of the pile.
     */
    pub fn get_pile_name(&self) -> String {
        return self.pile_name.clone();
    }

    // Each of the below card getter methods could likely be implemented using a immutable reference,
    // But a get method is used just in case.

    /**
     * get_card_description
     * Getter method for the card's description.
     */
    pub fn get_card_description(&self) -> String {
        let demo_card = (self.card_creator)();
        return demo_card.get_description().to_string();
    }

    /**
     * get_card_price
     * Getter method for the card's price.
     */
    pub fn get_card_price(&self) -> i32 {
        let demo_card = (self.card_creator)();
        return demo_card.get_cost();
    }

    /**
     * get_card_name
     * Getter method for card's name
     */
    pub(crate) fn get_card_name(&self) -> String {
        let demo_card = (self.card_creator)();
        return demo_card.get_name().to_string();
    }
}

