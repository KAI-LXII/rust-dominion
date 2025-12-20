/*
SPUStudnet
12/15/2024
card_properties.rs
File containing several struct compositions of card properties.
*/

use crate::card_manager::card_structures::Pile;

/**
 * CardTypes
 * Enum representing when cards can be played, represented by an integer.
 * Integer representation was critical for a function I didn't get to implement.
 */
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum CardTypes {
    Victory = 0,
    Treasure = 1,
    Action = 2,
    Reaction = 3,
    Attack = 4
}

/**
 * VictoryProperties
 * Struct representing the properties that each victory card must have.
 */
#[derive (Clone)]
pub struct VictoryProperties {
    // Points: how many points the card is worth.
    pub(crate) points: i32
}

/**
 * Implementation of victory properties.
 */
impl VictoryProperties {
    /**
     * get_points
     * Returns the number of points the victory card is work as an i32.
     * An i32 was picked as certain cards can result in negative victory points.
     */
    pub fn get_points(&self) -> i32 {
        return self.points;
    }
}

/**
 * TreasureProperties
 * Struct representing the properties that each treasure card must have.
 */
#[derive (Clone)]
pub struct TreasureProperties {
    // Value - How much buying power is in this card when it is played.
    pub(crate) value: i32
}
/**
 * Treasure properties implementation
 * Returns the value of the treasure.
 */
impl TreasureProperties {
    pub fn get_value(&self) -> i32 {
        return self.value; 
    }
}


#[derive (Clone, Hash, Eq, PartialEq)]
pub struct AttackProperties {
    pub(crate) attack_sequence: Vec<AttackSequence>
}

impl AttackProperties {
    pub fn new() -> AttackProperties {
        AttackProperties {
            attack_sequence: Vec::new()
        }

    }
}


/**
 * AttackSequence
 * Things, in order, that happen when an attack card is played.
 */
#[derive (Clone, Hash, Eq, PartialEq)]
pub enum AttackSequence {
    HandSize(i32),
    // Add this back in for gaining curses
    // ForceGain(&mut Pile),
    TopDeckTypeOrReveal(CardTypes, i32)
}

// Below this line is unused structs and methods that I hope to implement before the deadline.

/**
 * ActionProperties
 * The properties that each action card must use.
 * These properties should change the player's individual properties for that turn.
 */
#[derive (Clone)]
pub struct ActionProperties {
    pub(crate) added_buys: u8,
    pub(crate) added_actions: u8,
    pub(crate) added_cards: u8,
    pub(crate) temp_coin: u8,
    pub(crate) event: ActionEvents
}

/**
 * ActionProperties implementation
 */
impl ActionProperties {
    /**
    * New - Returns a new ActionProperties with all blanks: Any card derived from it will have no effect if the options are not modified.
    */
    pub fn new() -> ActionProperties {

        ActionProperties {
            added_buys: 0,
            added_actions: 0,
            added_cards: 0,
            temp_coin: 0,
            event: ActionEvents::No,

        }
    }
}

/**
 * ActionEvents
 * Different events, signaled to the game, that can happen.
 * These are required as certain cards require additional dialogue.
 */
#[derive(PartialEq, Clone, Copy)]
pub enum ActionEvents {
    Workshop,
    Merchant,
    No
}