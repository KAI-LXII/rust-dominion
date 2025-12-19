/*
SPUStudnet
12/15/2024
player_middleware.rs
A set of functions required to be called by the player, in order to accomplish things like logging, and playing action cards.
Currently unimplemented.
*/

use crate::player::player::Player;
use crate::card_manager::card::Card;

// This section of code is not currently in use.
// It is meant to log actions and do modifications in case of actions being played.
// Comments are available, if needed.

/**
 * PlayerMiddleware struct
 * Contains a game log for all actions in the game,
 * and an update log for all new actions since the last time the log was queried.
 */
pub(crate) struct PlayerMiddleware {
    game_log: Vec<String>,
    update_log: Vec<String>
}

/**
 * PlayerMiddleware implementation
 */
impl PlayerMiddleware {
    /**
     * Create a new playerMiddleWare.
     */
    pub (crate) fn new() -> PlayerMiddleware {
        let mut m = PlayerMiddleware {
            game_log: Vec::new(),
            update_log: Vec::new()
        };

        m.game_log.push(String::from("Game start!"));
        m.update_log.push(String::from("Game start!"));

        return m;
        
    }

    /**
     * Middleware for when the player plays a card.
     * Writes a message to the log.
     * Also signals for event-type affects to happen 
     */
    pub(crate) fn played_card_middleware(&mut self, p: &Player, card: &Box<dyn Card>) {
        let message = format!("{} played {}", p.name, card.get_name());

        self.game_log.push(message.clone());
        self.update_log.push(message.clone());
    }

    /**
     * Middleware for when the player buys a card.
     * Writes a message to the log.
     */
    pub(crate) fn bought_card_middleware(&mut self, name: String, card_name: String) {
        let message = format!("{} bought {}", name, card_name);

        self.game_log.push(message.clone());
        self.update_log.push(message.clone());
    }

    /**
     * Get log
     * Lazy implementation, if needed to get the game's entire log.
     */
    pub fn get_log(&self) -> Vec<String> {
        return self.game_log.clone();
    }

    /**
     * Get the update log.
     * Meant to be called by the UI layer.
     * Returns all new log updates, then clears the update log.
     */
    pub fn get_update_log(&mut self) -> Vec<String> {
        let log = self.update_log.clone();
        self.update_log.clear();
        return log;
    }
}