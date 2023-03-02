use crate::game_rules::GameRules;

pub struct Guesser {
    game_rules: GameRules,
}

impl Guesser {
    pub fn new(game_rules: GameRules) -> Self {
        Guesser {
            game_rules,
        }
    }
    pub fn predict (&self) -> Vec<String> {
        loop {
            let arr = self.get_prediction();
            if self.filter(arr.clone()) {
                return arr;
            }
            println!("Please enter a valid prediction of size {}!", self.game_rules.pattern_size);
        }
    }
    fn get_prediction (&self) -> Vec<String> {
        
        println!("Enter your guess: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input1 = input.trim();
        let input2: Vec<&str> = input1.split(" ").collect();
        let mut arr: Vec<String> = vec![];
        for i in 0..(input2.len() as usize){
            arr.push(input2[i].to_string());
        }
        arr

        // let mut arr: Vec<i32> = vec![];
        // for i in 0..(self.game_rules.pattern_size as usize){
        //     arr.push(input[i].parse().expect("Please type a number!"));
        // }
    }

    fn filter (&self, pred : Vec<String>) ->  bool {
        return pred.len() == (self.game_rules.pattern_size as usize);
    }

}