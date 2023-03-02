use std::hash::Hash;

// use rand::seq::SliceRandom;

use crate::game_rules::GameRules;
use super::pattern::Pattern;
use super::hint::Hint;

#[derive(Clone)]
pub struct Checker <T, const N: usize> 
where
    T: Eq + Hash + Clone
{
    pattern: Pattern<T,N>,
    hint: Hint<T,N>,
    game_rules: GameRules,
    pub current_trails: i32,
}

impl <T, const N:usize> Checker <T,N> 
where
    T: Eq + Hash + Clone
{
    pub fn new (arr: [T; N], game_rules: GameRules) -> Self {
        let hint = Hint::new(game_rules.hints_difficulty);
        let pattern = Pattern::new(arr, true);

        Checker {
            pattern,
            hint,
            game_rules,
            current_trails: 0,
        }
    }
    pub fn check (mut self, arr: [T; N]) -> i32 {
        let guess = Pattern::new(arr, false);
        if self.current_trails >= self.game_rules.no_of_trials {
            return -1;
        }
        if self.pattern.get_equality(&guess) {
            return 1;
        }
        self.hint.provide_hint(self.pattern, guess);
        0
    }
}
