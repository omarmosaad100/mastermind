/*
    GameRules:
        
        + NO of Trials,
        + Pattern Size,
        + Hints Difficulty, // 0 for easy, 1 for medium, 2 for hard
        + NO of Options, *
        + Type of Choices *

 */

enum TypeOfChoices {
    Colors,
}

struct GameRules {
    no_of_trials: i32,
    pattern_size: i32,
    hints_difficulty: i32,
    no_of_options: i32,
    type_of_choices: TypeOfChoices,
}

impl GameRules {
    fn new(no_of_trails : i32, pattern_size: i32, hints_difficulty: i32, no_of_options: i32, type_of_choice: TypeOfChoices) -> Self {
        GameRules {
            no_of_trails,
            pattern_size,
            hints_difficulty,
            no_of_options,
            type_of_choice
        }
    }
}