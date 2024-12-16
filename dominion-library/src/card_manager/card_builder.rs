/*
SPUStudnet
12/15/2024
card_builder.rs
Builder for several common cards used in dominion.
*/

use crate::player::phases::PlayerPhases;
use crate::card_manager::card::{Card, CardProperties, TypeProperties};
use crate::card_manager::card_properties::{ActionProperties, CardTypes, TreasureProperties, VictoryProperties, ActionEvents};


/**
 * build_copper
 * Builder method for copper.
 * Builds one copper card, and returns it in a box.
 * Only this first method is commented, as most of the rest is redundant.
 */
pub fn build_copper() -> Box<dyn Card> {
    // Create a cardproperties struct (Implements Card)
    let prop = CardProperties {
        // Set the name
        name: String::from("Copper"),
        // Set when it can be played.
        played_during: PlayerPhases::Buy,
        // Set its cost.
        cost: 0,
        // Set what kind of card it is (Action, treasure, action-attack, etc.)
        card_types: vec![CardTypes::Treasure],
        // Type properties: Based on what kind of card it is, it will have a set of properties
        // Explicitly related to its type. In this case, it is a treasure card, so it has treasure properties.
        card_type_properties: TypeProperties {
            treasure_properties: TreasureProperties {
                // How much the treasure is wroth when played.
                value: 1
            },
            // A raw description of the card, utilized to display extra information to the user.
            description: String::from(format!(r#"
                Copper
                Type: Treasure
                Cost: 0
                Buying power: 1
            "#))
            ,..Default::default()
        }

       
    };

    return Box::new(prop);
}

/**
 * build_silver
 * Builder method for silver.
 * Builds one silver card, and returns it in a box.
 */
pub fn build_silver() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Silver"),
        played_during: PlayerPhases::Buy,
        cost: 3,
        card_types: vec![CardTypes::Treasure],
        card_type_properties: TypeProperties {
            treasure_properties: TreasureProperties {
                value: 2
            },
            description: String::from(format!(r#"
                Silver
                Type: Treasure
                Cost: 3
                Buying power: 2
            "#))
            ,..Default::default()
        }

       
    };

    return Box::new(prop);
}

/**
 * build_gold
 * Builder method for gold.
 * Builds one gold card, and returns it in a box.
 */
pub fn build_gold() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Gold"),
        played_during: PlayerPhases::Buy,
        cost: 6,
        card_types: vec![CardTypes::Treasure],
        card_type_properties: TypeProperties {
            treasure_properties: TreasureProperties {
                value: 3
            },
            description: String::from(format!(r#"
                Gold
                Type: Treasure
                Cost: 6
                Buying power: 3
            "#))
            ,..Default::default()
        }

       
    };

    return Box::new(prop);
}

/**
 * build_estate
 * Builder method for estate.
 * Builds one estate card, and returns it in a box.
 */
pub fn build_estate() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Estate"),
        played_during: PlayerPhases::Never,
        cost: 2,
        card_types: vec![CardTypes::Victory],
        card_type_properties: TypeProperties {
            victory_properties: VictoryProperties {
                points: 1
            },
            description: String::from(format!(r#"
                Estate
                Type: Victory
                Cost: 2
                Point Value: 1
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);

}
/**
 * build_duchy
 * Builder method for duchy.
 * Builds one duchy card, and returns it in a box.
 */
pub fn build_duchy() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Duchy"),
        played_during: PlayerPhases::Never,
        cost: 5,
        card_types: vec![CardTypes::Victory],
        card_type_properties: TypeProperties {
            victory_properties: VictoryProperties {
                points: 3
            },
            description: String::from(format!(r#"
                Duchy
                Type: Victory
                Cost: 5
                Point Value: 3
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);

}
/**
 * build_province
 * Builder method for province.
 * Builds one province card, and returns it in a box.
 */
pub fn build_province() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Province"),
        played_during: PlayerPhases::Never,
        cost: 8,
        card_types: vec![CardTypes::Victory],
        card_type_properties: TypeProperties {
            victory_properties: VictoryProperties {
                points: 6
            },
            description: String::from(format!(r#"
                Province
                Type: Victory
                Cost: 8
                Point Value: 6
                Ends the game when all are bought.
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);

}

/**
 * Build smithy method.
 * Smith is a card that takes an action, but gives you three cards.
 */
pub fn build_smithy() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Smithy"),
        played_during: PlayerPhases::Action,
        cost: 4,
        card_types: vec![CardTypes::Action],
        card_type_properties: TypeProperties {
            action_properties: ActionProperties {
                added_buys: 0,
                added_actions: 0,
                added_cards: 3,
                temp_coin: 0,
                event: ActionEvents::No,
            },
            description: String::from(format!(r#"
                Smithy
                Type: Action
                Cost: 4
                + 3 Cards
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);
}


/**
 * Build village method.
 * Village is a card that gives you an extra action and replenishes your card.
 */
pub fn build_village() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Village"),
        played_during: PlayerPhases::Action,
        cost: 3,
        card_types: vec![CardTypes::Action],
        card_type_properties: TypeProperties {
            action_properties: ActionProperties {
                added_buys: 0,
                added_actions: 2,
                added_cards: 1,
                temp_coin: 0,
                event: ActionEvents::No,
            },
            description: String::from(format!(r#"
                Village
                Type: Action
                Cost: 3
                + 1 Card
                + 2 Actions
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);
}

/**
 * Build market method.
 * Market is a card that gives you a little bit of everything.
 */
pub fn build_market() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Market"),
        played_during: PlayerPhases::Action,
        cost: 5,
        card_types: vec![CardTypes::Action],
        card_type_properties: TypeProperties {
            action_properties: ActionProperties {
                added_buys: 1,
                added_actions: 1,
                added_cards: 1,
                temp_coin: 1,
                event: ActionEvents::No,
            },
            description: String::from(format!(r#"
                Market
                Type: Action
                Cost: 5
                + 1 Card
                + 1 Action
                + 1 Buy
                + 1 Coin
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);
}


/**
 * Build moat method.
 * Moat is a card that gives you defense, except your opponent never attacks so I'm not implementing it.
 */
pub fn build_moat() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Moat"),
        played_during: PlayerPhases::Action,
        cost: 2,
        card_types: vec![CardTypes::Action, CardTypes::Reaction],
        card_type_properties: TypeProperties {
            action_properties: ActionProperties {
                added_buys: 0,
                added_actions: 0,
                added_cards: 2,
                temp_coin: 0,
                event: ActionEvents::No,
            },
            description: String::from(format!(r#"
                Moat
                Type: Action-Reaction
                Cost: 2
                +2 Cards
                If you have this in your hand, you may reveal it to negate any attack card.
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);
}


/**
 * Build woodcutter method.
 */
pub fn build_woodcutter() -> Box<dyn Card> {
    let prop = CardProperties {
        name: String::from("Woodcutter"),
        played_during: PlayerPhases::Action,
        cost: 3,
        card_types: vec![CardTypes::Action],
        card_type_properties: TypeProperties {
            action_properties: ActionProperties {
                added_buys: 1,
                added_actions: 0,
                added_cards: 0,
                temp_coin: 2,
                event: ActionEvents::No,
            },
            description: String::from(format!(r#"
                Woodcutter
                Type: Action
                Cost: 3
                +2 Coins
                +1 Buy
            "#)),
            ..Default::default()
        }

    };

    return Box::new(prop);
}