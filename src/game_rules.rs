/*
    GameRules:
        
        + NO of Trials,
        + Pattern Size,
        + Hints Difficulty, // 0 for easy, 1 for medium, 2 for hard
        + NO of Options, *
        + Type of Choices *

 */

 #[derive(Debug)]
pub enum TypeOfChoices {
    Colors,
}

pub struct GameRules {
    pub no_of_trials: i32,
    pub pattern_size: i32,
    pub hints_difficulty: i32,
    pub no_of_options: i32,
    pub type_of_choices: TypeOfChoices,
}

impl GameRules {
    pub fn new(no_of_trials : i32, pattern_size: i32, hints_difficulty: i32, no_of_options: i32, type_of_choices: TypeOfChoices) -> Self {
        GameRules {
            no_of_trials,
            pattern_size,
            hints_difficulty,
            no_of_options,
            type_of_choices
        }
    }
    pub fn print(&self) {
        println!("Number of Trials: {}", self.no_of_trials);
        println!("Pattern Size: {}", self.pattern_size);
        println!("Hints Difficulty: {}", self.hints_difficulty);
        println!("Number of Options: {}", self.no_of_options);
        println!("Type of Choices: {:?}", self.type_of_choices);
    }
}