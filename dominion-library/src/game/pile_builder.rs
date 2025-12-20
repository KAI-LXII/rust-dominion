/*
SPUStudnet
12/15/2024
pile_builder.rs
File used to store methods for building piles of cards.
*/

use std::collections::VecDeque;
use crate::card_manager::card::Card;
use crate::card_manager::card_builder::*;
use crate::card_manager::card_structures::Pile;

/**
 * build_default_player_deck
 * Build the default player deck, three estates, seven coppers.
 */
pub(crate) fn build_default_player_deck() -> VecDeque::<Box<dyn Card>> {
    let mut new_default = VecDeque::new();
    for _ in 0..3 {
        new_default.push_front(build_estate())
    }

    for _ in 0..7 {
        new_default.push_front(build_copper())
    }

    return new_default;
}

/**
 * build_treasure_piles
 * Build the default treasure piles:
 * Copper, silver, and gold.
 */
pub(crate) fn build_treasure_piles() -> Vec<Pile> {
    let copper_pile = Pile {
        pile_name: String::from("Copper Pile"),
        cards_left: 46,
        card_creator: build_copper
    };

    let silver_pile = Pile {
        pile_name: String::from("Silver Pile"),
        cards_left: 40,
        card_creator: build_silver
    };

    let gold_pile = Pile {
        pile_name: String::from("Gold Pile"),
        cards_left: 30,
        card_creator: build_gold
    };

    return vec![copper_pile, silver_pile, gold_pile];

}

/**
 * build_victory_piles
 * Build the default victory piles:
 * Estates, Duchies, provinces.
 */
pub(crate) fn build_victory_piles() -> Vec<Pile> {
    let estate_pile = Pile {
        pile_name: String::from("Estate Pile"),
        cards_left: 8,
        card_creator: build_estate
    };

    let duchy_pile = Pile {
        pile_name: String::from("Duchy Pile"),
        cards_left: 8,
        card_creator: build_duchy
    };

    let province_pile = Pile {
        pile_name: String::from("Province Pile"),
        cards_left: 8,
        card_creator: build_province
    };

    return vec![estate_pile, duchy_pile, province_pile];

}

pub (crate) fn build_militia_pile() -> Pile {
    let militia_pile = Pile {
        pile_name: String::from("Militia pile"),
        cards_left: 10,
        card_creator: build_militia
    };

    return militia_pile
}

/**
 * Builder for smithy action card.
 */
pub (crate) fn build_smithy_pile() -> Pile {
    let smith_pile = Pile {
        pile_name: String::from("Smithy pile"),
        cards_left: 10,
        card_creator: build_smithy
    };

    return smith_pile;
}


/**
 * Builder for village action card.
 */
pub (crate) fn build_village_pile() -> Pile {
    let village_pile = Pile {
        pile_name: String::from("Village Pile"),
        cards_left: 10,
        card_creator: build_village
    };
    return village_pile
}

/**
 * Builder for market action card.
 */
pub (crate) fn build_market_pile() -> Pile {
    let market_pile = Pile {
        pile_name: String::from("Market Pile"),
        cards_left: 10,
        card_creator: build_market
    };
    return market_pile
}

/**
 * Builder for moat action card.
 */
pub (crate) fn build_moat_pile() -> Pile {
    let moat_pile = Pile {
        pile_name: String::from("Moat Pile"),
        cards_left: 10,
        card_creator: build_moat
    };
    return moat_pile
}


/**
 * Builder for woodcutter action card.
 */
pub (crate) fn build_woodcutter_pile() -> Pile {
    let woodcutter_pile = Pile {
        pile_name: String::from("Woodcutter Pile"),
        cards_left: 10,
        card_creator: build_woodcutter
    };
    return woodcutter_pile
}