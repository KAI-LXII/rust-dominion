/*
SPUStudnet
12/15/2024
game_errors.rs
Defines an enum with common game error types, commmunicated up to the UI to allow for user input.
*/

use std::error::Error;
use std::fmt;

/**
 * CardNotFoundError
 * Returns a message saying that it can't find a card at an index.
 * (Used for things like finding piles)
 */
#[derive(Debug)]
pub struct CardNotFoundError {
    pub index: usize
}

impl Error for CardNotFoundError {}

impl fmt::Display for CardNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card at index {} not found", self.index)
    }
}

/**
 * OutOfCardsError
 * Almost exclusively used for pile.
 */
#[derive(Debug, Clone)]
pub struct OutOfCardsError;

impl fmt::Display for OutOfCardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No more cards in object.")
    }
}

/**
 * InvalidActionError
 * Emitted whenever the player tries to do something they can't
 * (Playing cards out of order, etc.)
 */
#[derive(Debug)]
pub struct InvalidActionError {
    pub action_attempted: String
}

impl Error for InvalidActionError {}

impl fmt::Display for InvalidActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid action: {}", self.action_attempted)
    }
}

/**
 * Enum for encapsulation of errors, so the return can be generic.
 */
#[derive (Debug)]
pub enum GameErrors {
    OutOfCardsError(OutOfCardsError),
    CardNotFoundError(CardNotFoundError),
    InvalidActionError(InvalidActionError)
}

/**
 * From so that GameErrors can be easilly encapsulated and sent up the stack.
 */
impl From<OutOfCardsError> for GameErrors {
    fn from(value: OutOfCardsError) -> Self {
        Self::OutOfCardsError(value)
    }
}

impl From<CardNotFoundError> for GameErrors {
    fn from(value: CardNotFoundError) -> Self {
        Self::CardNotFoundError(value)
    }
}

impl From<InvalidActionError> for GameErrors {
    fn from(value: InvalidActionError) -> Self {
        Self::InvalidActionError(value)
    }
}
