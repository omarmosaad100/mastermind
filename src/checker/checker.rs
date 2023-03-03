use std::hash::Hash;

// use rand::seq::SliceRandom;

use super::super::game_rules::game_rules::GameRules;
use super::pattern::Pattern;
use super::hint::Hint;

#[derive(Clone)]
pub struct Checker <T> 
where
    T: Eq + Hash + Clone
{
    pub pattern: Pattern<T>,
    hint: Hint<T>,
    game_rules: GameRules,
    pub current_trails: i32,
}

impl <T> Checker <T> 
where
    T: Eq + Hash + Clone
{
    pub fn new (list: Vec<T>, game_rules: GameRules) -> Self {
        let hint = Hint::new(game_rules.hints_difficulty);
        let pattern = Pattern::new(list, true);

        Checker {
            pattern,
            hint,
            game_rules,
            current_trails: 1,
        }
    }
    pub fn check (mut self, list: Vec<T>) -> i32 {
        let guess = Pattern::new(list, false);
        if self.pattern.get_equality(&guess) {
            return 1;
        }
        
        if self.current_trails >= self.game_rules.no_of_trials {
            return -1;
        }

        self.hint.provide_hint(self.pattern, guess);
        0
    }
}
