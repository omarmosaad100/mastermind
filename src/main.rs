// This is the main entry point of the program

mod game_rules;
use game_rules::GameRules;

mod checker;
use checker::Checker;

mod guesser;
use guesser::Guesser;


fn get_input_game () -> (i32, i32, i32, i32){

    fn get_input () -> i32 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please type a number!")
    }
    
    let no_of_trials: i32;
    let pattern_size: i32;
    let hints_difficulty: i32;
    let no_of_options: i32;

    println!("Enter the number of trials: ");
    no_of_trials = get_input ();

    println!("Enter the pattern size: ");
    pattern_size = get_input ();

    println!("Enter the hints difficulty: ");
    hints_difficulty = get_input ();

    println!("Enter the number of options: ");
    no_of_options = get_input ();

    return (no_of_trials, pattern_size, hints_difficulty, no_of_options);

    // println!("Enter the type of choices: ");

}

fn create_game() -> GameRules {

    println!("******************************");

    let mut no_of_trials: i32 = 10;
    let mut pattern_size: i32 = 4;
    let mut hints_difficulty: i32 = 1;
    let mut no_of_options: i32 = 6;
    let type_of_choices: game_rules::TypeOfChoices = game_rules::TypeOfChoices::Colors;
    let mut game_rule: GameRules = GameRules::new(no_of_trials, pattern_size, hints_difficulty, no_of_options, type_of_choices);

    print!("DEFAULT GAME RULES:\n");
    game_rule.print();
    println!("******************************");


    println!("Do you want to change the default game rules? (y/n): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if input == "y" {
        (no_of_trials, pattern_size, hints_difficulty, no_of_options) = get_input_game();
        let type_of_choices: game_rules::TypeOfChoices = game_rules::TypeOfChoices::Colors;
        game_rule = GameRules::new(no_of_trials, pattern_size, hints_difficulty, no_of_options, type_of_choices);
    }
    println!("******************************");

    return game_rule;
}

fn play_game(game_rules: GameRules) {

    let arr = ["1", "2", "3", "4"];

    let mut checker = Checker::new(arr, game_rules.clone());

    let guesser = Guesser::new(game_rules.clone());

    while checker.current_trails <= game_rules.no_of_trials {
        
        println!("Times guessed: {}/{}", checker.current_trails, game_rules.no_of_trials);

        let guess = guesser.predict();
        let mut guess_arr: [&str; 4] = ["0", "0", "0", "0"];
        for i in 0..guess.len() {
            guess_arr[i] = &guess[i];
        }
        let response = checker.clone().check(guess_arr);
        checker.current_trails += 1;

        println!("");

        if response == 1 {
            println!("You won!");
            break;
        } else if response == -1 {
            println!("You lost!");
            break;
        }
    }

}

fn main() {
    println!("Welcome to Mastermind Game!\n");

    let game_rule: GameRules = create_game();

    println!("CURRENT GAME RULES:");
    game_rule.print();

    println!("******************************\n");

    println!("Game is starting...\n");

    play_game(game_rule);
}
