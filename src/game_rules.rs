/*
    GameRules:
        
        + NO of Trials,
        + Pattern Size,
        + Hints Difficulty, // 0 for easy, 1 for medium, 2 for hard
        + NO of Options, *
        + Type of Choices *

 */

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