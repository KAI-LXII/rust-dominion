/*
SPUStudnet
12/15/2024
player.rs
Player class.
Defines the complicated relationships, data types, and references needed to run a player.
*/

use crate::card_manager::card::Card;
use crate::card_manager::card_properties::CardTypes;
use crate::player::phases::PlayerPhases;
use crate::game::board::{Board, PlayerInterface};
use crate::game::game_errors::{GameErrors, InvalidActionError};
use crate::game::player_middleware::PlayerMiddleware;
use crate::game::board::CardSet;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::{
    cell::RefCell, rc::Rc
};


/**
 * Player struct
 * Structure containing all player data.
 */
pub struct Player {
    // Name of the player, currently unused.
    pub(crate) name: String,

    // The players deck (where they draw from)
    pub(crate) deck: VecDeque<Box<dyn Card>>,
    // The player's discard (Where cards go after they get used)
    pub(crate) discard: VecDeque<Box<dyn Card>>,
    // The player's hand (What they can play)
    pub(crate) hand: Vec<Box<dyn Card>>,
    // The cards of the player that are in play.
    pub(crate) in_play: Vec<Box<dyn Card>>,
    // What phase the player is in.
    pub(crate) phase: PlayerPhases,

    // A refCell to the board. used for purchasing cards.
    pub(crate) board: Rc<RefCell<Board>>,

    // Player Middleware refCell.
    pub(crate) player_middleware: Rc<RefCell<PlayerMiddleware>>,

    // Implement defaults for these two, and all parameters of Player.
    pub (crate) buy_power: u8,
    pub (crate) actions_remaining: u8,
    pub (crate) buys: u8

}

/**
 * Implementation of player methods.
 */
impl Player {
    /**
     * New method
     * Creates a new player object,
     * with most properties empty so they can be set.
     */
    pub(crate) fn new(deck: VecDeque<Box<dyn Card>>, board_ref: Rc<RefCell<Board>>, middle: Rc<RefCell<PlayerMiddleware>>) -> Player {
        return Player {
            name: String::from("No name supplied."),
            deck: deck,
            discard: VecDeque::<Box<dyn Card>>::new(),
            hand: Vec::<Box<dyn Card>>::new(),
            in_play: Vec::<Box<dyn Card>>::new(),
            phase: PlayerPhases::Idle,
            board: board_ref,
            player_middleware: middle,


            buy_power: 0,
            actions_remaining: 1,
            buys: 1
        };
    }

    /**
     * cleanup_and_draw
     * Cleanup the player's playing area, and draw five new cards from the deck.
     */
    pub(crate) fn cleanup_and_draw(&mut self) {
        self.phase = PlayerPhases::Idle;

        // Push all cards from the player's hand to the discard.
        while self.hand.len() > 0 {
            self.discard.push_front(self.hand.pop().expect("No more cards in hand to discard"));
        }

        // Push all cards from the player's playing area to the discard.
        while self.in_play.len() > 0 {
            self.discard.push_front(self.in_play.pop().expect("No more cards in play to discard"));
        }

        // Pick up five cards.
        for _ in 0..5 {
            self.draw_card();
        }

        // Reset player properties to pre-playing for next turn.
        self.buy_power = 0;
        self.actions_remaining = 1;
        self.phase = PlayerPhases::Idle;
        self.buys = 1;
    }

    /**
     * play_card
     * Play a card given an index.
     * Moves a card from the hand to the playing area, and applies its properties.
     */
    pub (crate) fn play_card(&mut self, hand_index: usize) -> Option<GameErrors> {
        let card = self.hand.get(hand_index).unwrap();

        if card.get_playing_phase() == &self.phase {

            let card = self.hand.remove(hand_index);
            if card.get_card_types().contains(&CardTypes::Treasure) {
                self.buy_power += card.get_relevant_value() as u8;

            } else if card.get_card_types().contains(&CardTypes::Action) {
                if self.actions_remaining >= 1 {
                    let props = card.get_action_properties();

                    self.actions_remaining += props.added_actions;

                    self.buys += props.added_buys;

                    self.buy_power += props.temp_coin;

                    for _ in 0.. props.added_cards {
                        self.draw_card();
                    }
                    self.actions_remaining -= 1;

                }
                else {
                    self.hand.push(card);
                    return Some(GameErrors::InvalidActionError(InvalidActionError {action_attempted: String::from("Attempted to play card without actions.") }))
                }

            }
            self.in_play.push(card);
            //self.player_middleware.borrow_mut().played_card_middleware(card);


            None
        }
        else {
            Some(GameErrors::InvalidActionError(InvalidActionError { action_attempted: String::from("You cannot play a that card at this time.")}))
        }
    }

    /**
     * buy_card
     * Buy a card from the board given an index and a cardset.
     * Deducts from buys and buy power as needed.
     * Returns an error if there is one.
     */
    pub (crate) fn buy_card(&mut self, pile_index: usize, c:CardSet) -> Option<GameErrors> {
        let price = self.board.borrow_mut().get_card_price(pile_index, c.clone());

        match price {
            Ok(x) => {
                if (self.buy_power as i32) < x || self.buys == 0 {
                    return Some(GameErrors::InvalidActionError(InvalidActionError {action_attempted: String::from("You are out of buys, or out of money. Wait until next turn!")}))
                }
            }
            Err(error) => {
                return Some(error);
            }
        }


        let result = self.board.borrow_mut().buy_card(pile_index, c.clone());
        self.buys -= 1;
        self.buy_power -= price.unwrap() as u8;

        match result {
            Ok(x) => {
                self.player_middleware.borrow_mut().bought_card_middleware(self.name.clone(),x.get_name().clone());
                self.discard.push_front(x);
                return None;
            }
            Err(error) => {
                return Some(error);
            }
        }
    }

    /**
     * Shuffle the deck and prepend the discard (behind the deck)
     */
    fn shuffle_and_prepend_discard(&mut self) {
        // Make the discard contiguous so it can be shuffled using thread_rng.
        self.discard.make_contiguous().shuffle(&mut thread_rng());

        // While there's still cards in the discard, push them into the deck at the back.
        while self.discard.len() > 0 {
            self.deck.push_back(self.discard.pop_front().expect("No more cards from discard!"))
        }
    }

    /**
     * draw_card
     * Draw a card from the deck.
     */
    pub(crate) fn draw_card(&mut self) {
        // Get a card from the deck.
        let option_card = self.deck.pop_front();

        // match the card, so if it doesn't exist, we can shuffle the discard.
        match option_card {
            Some(card) => {
                // Push the card into the hand.
                self.hand.push(card);
            }
            None => {
                self.shuffle_and_prepend_discard();
                // If there's no cards, shuffle the deck, then try to pick up a card.
                if self.deck.len() != 0 {
                    // Get the card, push it into the hand.
                    let guarentee_card = self.deck.pop_front().expect("This should never happen: Added a card after making sure cards were available.");
                    self.hand.push(guarentee_card);
                    
                }
                // If there's no more cards in the deck after shuffling the discard, don't draw anything.
            }
        }
        
    }

    /**
     * play_treasures
     * Play all treasures in hand.
     */
    pub (crate) fn play_treasures(&mut self) {
        // initialize a counter (so that as items are removed we keep track.)
        let mut i = 0;
        // While we aren't at the end of the hand.
        while  i < self.get_hand().len() {

            // Get the card.
            let c = self.get_hand().get(i);
            match c {
                Some(c) => {
                    // If it's a treasure, play it.
                    if c.get_card_types().contains(&CardTypes::Treasure) {
                        self.play_card(i);
                    }
                    else {
                        // If it's not a treasure, move to the next card.
                        i += 1;
                    }
                }
                _ => {
                    // If there's no card, break out of the loop, as we're at the end.
                    break
                }
            }
            

        }
    }
    
    /**
     * Shuffle the deck
     * used at the start of the game.
     */
    pub(crate) fn shuffle_deck(&mut self) {
        // VecDeques are structs that efficiently insert at both the front and back of the vector.
        // This makes it helpful for decks in dominion, as you are required to insert at the front and back for multiple cards.
        // However, the side effect is that they need to be made into normal, contiguous vectors to be shuffled using the thread_rng.
        self.deck.make_contiguous().shuffle(&mut thread_rng());
    }

    /**
     * Advance_phase
     * Used by the player to skip through a phase.
     */
    pub(crate) fn advance_phase(&mut self) {
        match self.phase {
            PlayerPhases::Idle => {
                self.phase = PlayerPhases::Action;
            }
            PlayerPhases::Action => {
                self.phase = PlayerPhases::Buy;
            }
            PlayerPhases::Buy => {
                self.phase = PlayerPhases::Cleanup;
                self.cleanup_and_draw();
            }
            PlayerPhases::Cleanup => {
                self.phase = PlayerPhases::Idle;
            }
            _ => {}
        }
    }

    /**
     * end_turn
     * Ends your turn.
     */
    fn end_turn(&mut self) {
        self.cleanup_and_draw();
    }

    /**
     * count_victory_points
     * Counts the number of victory points in your entire hand.
     * Functions as a sort of scoreboard.
     */
    fn count_victory_points(&self) -> i32 {
        let mut vp : i32 = 0;
        for i in &self.deck {
            if i.get_card_types().contains(&CardTypes::Victory) {
                vp += i.get_relevant_value();
            }
        }
        for i in &self.hand {
            if i.get_card_types().contains(&CardTypes::Victory) {
                vp += i.get_relevant_value();
            }
        }
        for i in &self.discard {
            if i.get_card_types().contains(&CardTypes::Victory) {
                vp += i.get_relevant_value();
            }
        }
        return vp;
    }
}

/**
 * PlayerUIInterface
 * Wrapper methods handed to the UI to allow for access.
 * The effectiveness of this design choice is still in question.
 */
pub trait PlayerUIInterface {
    fn play_ui_card(&mut self, index: usize) -> Option<GameErrors>;

    fn buy_ui_card(&mut self, index: usize, set:CardSet) -> Option<GameErrors>;

    fn advance_ui_phase(&mut self);

    fn get_hand(&self) -> &Vec<Box<dyn Card>>;

    fn get_actions(&self) -> u8;

    fn get_buying_power(&self) -> u8;

    fn get_victory_points(&self) -> i32;
    
    fn get_buys(&self) -> u8;

    fn end_turn(&mut self);

    fn get_phase(&self) -> PlayerPhases;
    
    fn play_all_treasures_ui(&mut self);
}

impl PlayerUIInterface for Player {
    fn get_hand(&self) -> &Vec<Box<dyn Card>> {
        return &self.hand;
    }

    fn play_ui_card(&mut self, index: usize) -> Option<GameErrors> {
        return self.play_card(index);
    }

    fn buy_ui_card(&mut self, index: usize, set: CardSet) -> Option<GameErrors> {
        self.buy_card(index, set)
    }

    fn advance_ui_phase(&mut self) {
        if self.phase == PlayerPhases::Action || self.phase == PlayerPhases::Buy {
            self.advance_phase();
        }
    }

    fn end_turn(&mut self) {
        if self.phase == PlayerPhases::Action || self.phase == PlayerPhases::Buy {
            self.end_turn();
        }
    }
    
    fn get_actions(&self) -> u8 {
        return self.actions_remaining;
    }

    fn get_buying_power(&self) -> u8 {
        return self.buy_power;
    }

    fn get_victory_points(&self) -> i32 {
        return self.count_victory_points();
    }

    fn get_phase(&self) -> PlayerPhases {
        return self.phase.clone();
    }

    fn play_all_treasures_ui(&mut self) {
        return self.play_treasures();
    }

    fn get_buys(&self) -> u8 {
        return self.buys.clone();
    }
}

// Below this is unused methods for action cards.
// Players would recieve a "ResolveCardRequestTicket"
// For each action, and would change parameters as needed.

struct PlayerResolveCardRequestTicket {
    actions_gained: Option<u8>,
    cards_gained: Option<u8>,
    trash_cards: Option<u8>,
    gain_card_options: Option<Vec<Box<dyn Card>>>,
    discard_cards: Option<u8>
}

struct TurnModifier {

}