/*
SPUStudnet
12/15/2024
card.rs
Primary card structure
Creates a card impl that can be dynamically allocated and passed as a box with properties and methods.
*/

use crate::player::phases::PlayerPhases;
use crate::card_manager::card_properties::*;

/**
 * CardProperties
 * The basic properties of a card: common accross all cards.
 */
#[derive (Clone)]
pub struct CardProperties {
    pub name: String,
    pub(crate) played_during: PlayerPhases,
    pub(crate) cost: i32,
    pub(crate) card_types: Vec<CardTypes>,
    pub(crate) card_type_properties: TypeProperties

}

/**
 * TypeProperties
 * Additional property panel added to CardProperties.
 * Contains the properties of each type of card, as required by composition over inheritance.
 * There's likely a better way to do this, but I don't know it, and didn't have time to learn it.
 */
#[derive (Clone)]
pub struct TypeProperties {
    pub(crate) treasure_properties: TreasureProperties,
    pub(crate) victory_properties: VictoryProperties,
    pub(crate) action_properties: ActionProperties,
    pub(crate) description: String
}
/**
 * TypeProperties implementation
 * Generates a default TypeProperties for a card.
 */
impl Default for TypeProperties {
    fn default() -> TypeProperties {
        TypeProperties {
            treasure_properties: TreasureProperties {
                value: 0
            },
            victory_properties: VictoryProperties {
                points: 0
            },
            action_properties: ActionProperties::new(),
            description: String::from("No description provided"),
        }
    }

}

/**
 * Card trait
 * We pass a trait rather than an object because a trait can be "Boxed"
 * and passed around as a dynamic size at runtime.
 */
pub trait Card {
    fn get_playing_phase(&self) -> &PlayerPhases;
    fn get_cost(&self) -> i32;
    fn get_card_types(&self) -> &Vec<CardTypes>;
    fn get_relevant_value(&self) -> i32;
    fn get_description(&self) -> &String;
    fn get_name(&self) -> &String;
    fn get_action_properties(&self) -> ActionProperties;
}

/**
 * Implementing its getter methods for card properties.
 * Each of these are getter methods to hide the card data, as we want it to be immutable after creation.
 */
impl Card for CardProperties {
    fn get_playing_phase(&self) -> &PlayerPhases {
        return &self.played_during
    }

    fn get_cost(&self) -> i32 {
        return self.cost
    }

    fn get_card_types(&self) -> &Vec<CardTypes> {
        return &self.card_types;
    }

    fn get_relevant_value(&self) -> i32 {
        if self.card_types.contains(&CardTypes::Treasure) {
            return self.card_type_properties.treasure_properties.get_value()
        }
        else if self.card_types.contains(&CardTypes::Victory) {
            self.card_type_properties.victory_properties.get_points()
        }
        else {
            return 0;
        }
    }

    fn get_action_properties(&self) -> ActionProperties {
        return self.card_type_properties.action_properties.clone();
    }

    fn get_description(&self) -> &String {
        return &self.card_type_properties.description;
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }
}

