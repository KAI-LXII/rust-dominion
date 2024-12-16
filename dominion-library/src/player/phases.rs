/*
SPUStudnet
12/15/2024
phases.rs
Defines the phases that a player goes through in each game.
*/

/**
 * The phases, or "states" that a player is in at any time.
 * Idle is for when the opponent is playing
 * Action is for when actions can be played
 * Buy is for treasures and buying cards.
 * Cleanup is for discarding and drawing anew,
 * Never is a flag value for cards like "Victory" which will never be played and need a playerphase.
 */
#[derive(Eq, PartialEq, Clone)]
pub enum PlayerPhases {
    Idle = -1,
    Action,
    Buy,
    Cleanup,
    Never
}

/**
 * PlayerPhases to_string implementation.
 */
impl PlayerPhases {
    pub fn to_string(&self) -> String {
        match self {
            PlayerPhases::Action => {
                return String::from("Actions");
            }
            PlayerPhases::Buy => {
                return String::from("Buy");
            }
            PlayerPhases::Cleanup => {
                return String::from("Cleanup");
            }
            PlayerPhases::Idle => {
                return String::from("Waiting for opponent. . . ");
            }
            PlayerPhases::Never => {
                return String::from("This should never exist!");
            }
        }
    }
}